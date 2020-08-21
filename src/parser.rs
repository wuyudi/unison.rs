use byteorder::{BigEndian, ByteOrder};
use log::info;
use std::fs::File;
use std::io::Read;

use super::base32hex;

pub struct Buffer {
    buf: Vec<u8>,
    idx: usize,
    indent: usize,
}

impl Buffer {
    pub fn from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let mut buf = vec![];
        let mut file = File::open(path)?;
        file.read_to_end(&mut buf)?;
        Ok(Buffer {
            buf,
            idx: 0,
            indent: 0,
        })
    }

    fn get_n(&mut self, count: usize) -> &[u8] {
        self.idx += count;
        let res = &self.buf[self.idx - count..self.idx];
        res
    }

    pub fn get_term(&mut self) -> ABT<Term> {
        self.get()
    }

    fn get<T: FromBuffer>(&mut self) -> T {
        info!(
            "{}Getting {:?} @ {}",
            indent(self.indent),
            std::any::type_name::<T>(),
            self.idx
        );
        T::get(self)
    }

    fn get_with_env<T: FromBufferWithEnv>(&mut self, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> T {
        info!(
            "{}Getting with env {:?} @ {}",
            indent(self.indent),
            std::any::type_name::<T>(),
            self.idx
        );
        T::get(self, env, fvs)
    }
}

fn indent(n: usize) -> String {
    let mut res = "".to_owned();
    for _ in 0..n {
        res += "|  ";
    }
    res
}

trait FromBuffer {
    fn get(buf: &mut Buffer) -> Self;
}

impl FromBuffer for String {
    fn get(buf: &mut Buffer) -> Self {
        let length = usize::get(buf);
        let raw = buf.get_n(length);
        let res = String::from_utf8(raw.to_owned()).unwrap();
        info!("{}String: {}", indent(buf.indent), res);
        res
    }
}

impl FromBuffer for char {
    fn get(buf: &mut Buffer) -> Self {
        // String::from_utf8(buf.get::<usize>().to_owned())
        buf.get_n(1)[0] as char
    }
}

impl FromBuffer for u8 {
    fn get(buf: &mut Buffer) -> Self {
        buf.get_n(1)[0]
    }
}

impl FromBuffer for i64 {
    fn get(buf: &mut Buffer) -> Self {
        let data = buf.get_n(8);
        return BigEndian::read_u64(&data) as i64;
    }
}

impl FromBuffer for u64 {
    fn get(buf: &mut Buffer) -> Self {
        let data = buf.get_n(8);
        return BigEndian::read_u64(&data);
    }
}

impl FromBuffer for f64 {
    fn get(buf: &mut Buffer) -> Self {
        let data = buf.get_n(8);
        return BigEndian::read_u64(&data) as f64;
    }
}

impl FromBuffer for bool {
    fn get(buf: &mut Buffer) -> Self {
        let data: u8 = buf.get();
        return data == 1;
    }
}

impl FromBuffer for usize {
    fn get(buf: &mut Buffer) -> Self {
        // Varint, described here https://developers.google.com/protocol-buffers/docs/encoding#varints
        let word8 = u8::get(buf);
        info!("{}Getting usize buffer {:x}", indent(buf.indent), word8);
        let indicator = word8 >> 7;

        let res = if indicator == 0 {
            word8 as usize
        } else {
            let word8 = word8 ^ 128; // clear the indicator
            let rest = usize::get(buf);
            (rest << 7) + word8 as usize
        };

        info!("{}usize: {}", indent(buf.indent), res);
        res
    }
}

impl<T: FromBuffer> FromBuffer for Vec<T> {
    fn get(buf: &mut Buffer) -> Self {
        let mut res = vec![];
        let length = usize::get(buf);
        // info!("vec ({} elements)", length);
        for _ in 0..length {
            res.push(T::get(buf));
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    num: usize,
    text: String,
}

impl FromBuffer for Symbol {
    fn get(buf: &mut Buffer) -> Self {
        Symbol {
            num: buf.get(),
            text: buf.get(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConstructorType {
    Data,
    Effect,
}

impl FromBuffer for ConstructorType {
    fn get(buf: &mut Buffer) -> Self {
        let tag: u8 = buf.get();
        match tag {
            0 => ConstructorType::Data,
            1 => ConstructorType::Effect,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Hash(Vec<u8>);

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&base32hex::encode(&self.0))
    }
}

impl FromBuffer for Hash {
    fn get(buf: &mut Buffer) -> Self {
        let len: usize = buf.get();
        let res = Hash(buf.get_n(len).to_owned());
        info!("{}Hash: #{:?}", indent(buf.indent), res);
        res
    }
}

#[derive(Debug, Clone)]
pub struct Id(Hash, usize, usize);

impl FromBuffer for Id {
    fn get(buf: &mut Buffer) -> Self {
        Id(buf.get(), buf.get(), buf.get())
    }
}

#[derive(Debug, Clone)]
pub enum Reference {
    Builtin(String),
    DerivedId(Id),
}

impl FromBuffer for Reference {
    fn get(buf: &mut Buffer) -> Self {
        let tag = buf.get();
        match tag {
            0_u8 => Reference::Builtin(buf.get()),
            1 => Reference::DerivedId(buf.get()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Referent {
    Ref(Reference),
    Con(Reference, usize, ConstructorType),
}

impl FromBuffer for Referent {
    fn get(buf: &mut Buffer) -> Self {
        let tag = buf.get();
        info!("{}Referent tag : {}", indent(buf.indent), tag);
        match tag {
            0_u8 => Referent::Ref(buf.get()),
            1 => Referent::Con(buf.get(), buf.get(), buf.get()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchCase(Pattern, Option<Box<ABT<Term>>>, Box<ABT<Term>>);

impl FromBufferWithEnv for MatchCase {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        MatchCase(
            buf.get(),
            buf.get_with_env(env, fvs),
            buf.get_with_env(env, fvs),
        )
    }
}

#[derive(Debug, Clone)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

impl FromBuffer for Kind {
    fn get(buf: &mut Buffer) -> Self {
        let tag: u8 = buf.get();
        match tag {
            0 => Kind::Star,
            1 => Kind::Arrow(buf.get(), buf.get()),
            _ => unreachable!("Kind tag {}", tag),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Unbound,
    Var,
    Boolean(bool),
    Int(i64),
    Nat(u64),
    Float(f64),
    Text(String),
    Char(char),
    Cosntructor(Reference, usize, Vec<Pattern>),
    As(Box<Pattern>),
    EffectPure(Box<Pattern>),
    EffectBind(Reference, usize, Vec<Pattern>, Box<Pattern>),
    SequenceLiteral(Vec<Pattern>),
    SequenceOp(Box<Pattern>, SeqOp, Box<Pattern>),
}

impl FromBuffer for Pattern {
    fn get(buf: &mut Buffer) -> Self {
        let tag: u8 = buf.get();
        info!("Pattern branch {}", tag);
        match tag {
            0 => Pattern::Unbound,
            1 => Pattern::Var,
            2 => Pattern::Boolean(buf.get()),
            3 => Pattern::Int(buf.get()),
            4 => Pattern::Nat(buf.get()),
            5 => Pattern::Float(buf.get()),
            6 => Pattern::Cosntructor(buf.get(), buf.get(), buf.get()),
            7 => Pattern::As(buf.get()),
            8 => Pattern::EffectPure(buf.get()),
            9 => Pattern::EffectBind(buf.get(), buf.get(), buf.get(), buf.get()),
            10 => Pattern::SequenceLiteral(buf.get()),
            11 => Pattern::SequenceOp(buf.get(), buf.get(), buf.get()),
            12 => Pattern::Text(buf.get()),
            13 => Pattern::Char(buf.get()),
            _ => unreachable!("Kind tag {}", tag),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SeqOp {
    Cons,
    Snoc,
    Concat,
}

impl FromBuffer for SeqOp {
    fn get(buf: &mut Buffer) -> Self {
        let tag: u8 = buf.get();
        match tag {
            0 => SeqOp::Cons,
            1 => SeqOp::Snoc,
            2 => SeqOp::Concat,
            _ => unreachable!("Kind tag {}", tag),
        }
    }
}

// Base functor for types in the Unison language
#[derive(Debug, Clone)]
pub enum Type {
    Ref(Reference),
    Arrow(Box<ABT<Type>>, Box<ABT<Type>>),
    Ann(Box<ABT<Type>>, Kind),
    App(Box<ABT<Type>>, Box<ABT<Type>>),
    Effect(Box<ABT<Type>>, Box<ABT<Type>>),
    Effects(Vec<ABT<Type>>),
    Forall(Box<ABT<Type>>),
    //  binder like ∀, used to introduce variables that are
    //  bound by outer type signatures, to support scoped type
    //  variables
    IntroOuter(Box<ABT<Type>>),
}

#[derive(Debug, Clone)]
pub enum Term {
    Int(i64),
    Nat(u64),
    Float(f64),
    Boolean(bool),
    Text(String),
    Char(char),
    Blank,
    Ref(Reference),

    Constructor(Reference, usize),
    Request(Reference, usize),
    Handle(Box<ABT<Term>>, Box<ABT<Term>>),
    App(Box<ABT<Term>>, Box<ABT<Term>>),
    Ann(Box<ABT<Term>>, ABT<Type>),
    Sequence(Vec<Box<ABT<Term>>>),
    If(Box<ABT<Term>>, Box<ABT<Term>>, Box<ABT<Term>>),
    And(Box<ABT<Term>>, Box<ABT<Term>>),
    Or(Box<ABT<Term>>, Box<ABT<Term>>),
    Lam(Box<ABT<Term>>),
    //   -- Note: let rec blocks have an outer ABT.Cycle which introduces as many
    //   -- variables as there are bindings
    LetRec(bool, Vec<Box<ABT<Term>>>, Box<ABT<Term>>),
    //   -- Note: first parameter is the binding, second is the expression which may refer
    //   -- to this let bound variable. Constructed as `Let b (abs v e)`
    Let(bool, Box<ABT<Term>>, Box<ABT<Term>>),
    //   -- Pattern matching / eliminating data types, example:
    //   --  case x of
    //   --    Just n -> rhs1
    //   --    Nothing -> rhs2
    //   --
    //   -- translates to
    //   --
    //   --   Match x
    //   --     [ (Constructor 0 [Var], ABT.abs n rhs1)
    //   --     , (Constructor 1 [], rhs2) ]
    Match(Box<ABT<Term>>, Vec<MatchCase>),
    TermLink(Referent),
    TypeLink(Reference),
}

trait FromBufferWithEnv {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self;
}

impl<T: FromBuffer> FromBuffer for Box<T> {
    fn get(buf: &mut Buffer) -> Self {
        T::get(buf).into()
    }
}

impl<T: FromBufferWithEnv> FromBufferWithEnv for Box<T> {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        T::get(buf, env, fvs).into()
    }
}

impl<T: FromBufferWithEnv> FromBufferWithEnv for Option<T> {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        let tag = buf.get();
        match tag {
            0_u8 => None,
            1 => Some(T::get(buf, env, fvs).into()),
            _ => unreachable!("Option tag {}", tag),
        }
    }
}

impl<T: FromBufferWithEnv> FromBufferWithEnv for Vec<T> {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        let mut res = vec![];
        let length = usize::get(buf);
        for _ in 0..length {
            res.push(T::get(buf, env, fvs));
        }
        res
    }
}

impl FromBufferWithEnv for Type {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        let tag = u8::get(buf);
        info!("Type ({})", tag);
        match tag {
            0 => Type::Ref(buf.get()),
            1 => Type::Arrow(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            2 => Type::Ann(buf.get_with_env(env, fvs), buf.get()),
            3 => Type::App(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            4 => Type::Effect(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            5 => Type::Effects(buf.get_with_env(env, fvs)),
            6 => Type::Forall(buf.get_with_env(env, fvs)),
            7 => Type::IntroOuter(buf.get_with_env(env, fvs)),
            _ => unreachable!("Type tag {}", tag),
        }
    }
}

impl FromBufferWithEnv for Term {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        let tag = u8::get(buf);
        info!("{}Term {}", indent(buf.indent), tag);
        match tag {
            0 => Term::Int(buf.get()),
            1 => Term::Nat(buf.get()),
            2 => Term::Float(buf.get()),
            3 => Term::Boolean(buf.get()),
            4 => Term::Text(buf.get()),
            5 => Term::Ref(buf.get()),
            6 => Term::Constructor(buf.get(), buf.get()),
            7 => Term::Request(buf.get(), buf.get()),
            8 => Term::Handle(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            9 => Term::App(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            10 => Term::Ann(buf.get_with_env(env, fvs), buf.get()),
            11 => Term::Sequence(buf.get_with_env(env, fvs)),
            12 => Term::If(
                buf.get_with_env(env, fvs),
                buf.get_with_env(env, fvs),
                buf.get_with_env(env, fvs),
            ),
            13 => Term::And(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            14 => Term::Or(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            15 => Term::Lam(buf.get_with_env(env, fvs)),
            16 => Term::LetRec(
                false,
                buf.get_with_env(env, fvs),
                buf.get_with_env(env, fvs),
            ),
            17 => Term::Let(
                false,
                buf.get_with_env(env, fvs),
                buf.get_with_env(env, fvs),
            ),
            18 => Term::Match(buf.get_with_env(env, fvs), buf.get_with_env(env, fvs)),
            19 => Term::Char(buf.get()),
            20 => Term::TermLink(buf.get()),
            21 => Term::TypeLink(buf.get()),
            _ => panic!("Failed {}", tag),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ABT<Content> {
    Var(Symbol),
    Cycle(Box<ABT<Content>>),
    Abs(Symbol, Box<ABT<Content>>),
    Tm(Content),
}

impl<Inner: FromBufferWithEnv + std::fmt::Debug> FromBufferWithEnv for ABT<Inner> {
    fn get(buf: &mut Buffer, env: &Vec<Symbol>, fvs: &Vec<Symbol>) -> Self {
        let tag: u8 = buf.get();
        info!("{}ABT ({})", indent(buf.indent), tag);
        buf.indent += 1;
        let res = match tag {
            0 => {
                let tag = u8::get(buf);
                match tag {
                    0 => ABT::Var(env[buf.get::<usize>()].clone()),
                    1 => ABT::Var(fvs[buf.get::<usize>()].clone()),
                    _ => unreachable!(),
                }
            }
            1 => ABT::Tm(Inner::get(buf, env, fvs)),
            2 => {
                let v: Symbol = buf.get();
                let mut nw = env.to_owned();
                nw.insert(0, v.clone());
                ABT::Abs(v, buf.get_with_env(&nw, fvs))
            }
            3 => ABT::Cycle(buf.get_with_env(env, fvs)),
            _ => unreachable!("ABT {}", tag),
        };
        info!("{}ABT: {:?}", indent(buf.indent), res);
        buf.indent -= 1;
        res
    }
}

impl FromBuffer for ABT<Type> {
    fn get(buf: &mut Buffer) -> Self {
        let names = Vec::<Symbol>::get(buf);
        info!("{}FVS: {:?}", indent(buf.indent), names);
        buf.get_with_env(&vec![], &names)
    }
}

impl FromBuffer for ABT<Term> {
    fn get(buf: &mut Buffer) -> Self {
        let names = Vec::<Symbol>::get(buf);
        info!("{}FVS: {:?}", indent(buf.indent), names);
        buf.get_with_env(&vec![], &names)
    }
}
