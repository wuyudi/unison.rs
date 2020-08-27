use im::Vector;
use serde_derive::{Deserialize, Serialize};
use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Symbol {
    pub num: usize,
    pub text: String,
    pub unique: usize,
}
impl std::fmt::Debug for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("🔣{}/{}", self.text, self.unique))
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
    PartialOrd,
)]
pub enum ConstructorType {
    Data,
    Effect,
}

#[derive(
    Serialize, Deserialize, Clone, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash, PartialOrd,
)]
pub enum Reference {
    Builtin(String),
    DerivedId(Id),
}

impl Reference {
    pub fn from_hash(hash: &str) -> Self {
        Reference::DerivedId(Id(Hash::from_string(hash), 0, 1))
    }
}

#[derive(
    Serialize, Deserialize, Clone, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash, PartialOrd,
)]
pub struct Hash(pub Vec<u8>);

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
    PartialOrd,
)]
pub struct Id(pub Hash, pub usize, pub usize);

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
    PartialOrd,
)]
pub enum Referent {
    Ref(Reference),
    Con(Reference, usize, ConstructorType),
}

// impl Referent {
//     fn reference(&self) -> &Reference {
//         match self {
//             Referent::Ref(r) => r,
//             Referent::Con(r, _, _) => r,
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchCase(pub Pattern, pub Option<Box<ABT<Term>>>, pub Box<ABT<Term>>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Pattern {
    Unbound,
    Var,
    Boolean(bool),
    Int(i64),
    Nat(u64),
    Float(f64),
    Text(String),
    Char(char),
    Constructor(Reference, usize, Vec<Pattern>),
    As(Box<Pattern>),
    EffectPure(Box<Pattern>),
    EffectBind(Reference, usize, Vec<Pattern>, Box<Pattern>),
    SequenceLiteral(Vec<Pattern>),
    SequenceOp(Box<Pattern>, SeqOp, Box<Pattern>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum SeqOp {
    Cons,
    Snoc,
    Concat,
}

// Base functor for types in the Unison language
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

// pub type Rc<Value> = usize;

// Runtime values
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    Int(i64),
    Nat(u64),
    Float(f64),
    Boolean(bool),
    Text(String),
    Bytes(Vec<u64>),
    Char(char),
    Ref(Reference),

    CycleFnBody(
        usize,
        Vec<(Symbol, usize, Rc<Value>)>,
        Vec<(Symbol, usize, usize)>,
    ),
    PartialFnBody(usize, Vec<(Symbol, usize, Rc<Value>)>),
    PartialNativeApp(String, Vec<Rc<Value>>),
    PartialConstructor(Reference, usize, Vector<Rc<Value>>),

    Continuation(usize, Vec<super::ir_runtime::Frame>),
    Constructor(Reference, usize),
    Request(Reference, usize),
    RequestPure(Rc<Value>),
    RequestWithArgs(Reference, usize, usize, Vec<Rc<Value>>),
    RequestWithContinuation(
        Reference,
        usize,
        Vec<Rc<Value>>,
        usize,
        Vec<super::ir_runtime::Frame>,
    ),

    Sequence(Vector<Rc<Value>>),
    TermLink(Referent),
    TypeLink(Reference),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Term {
    Int(i64),
    Nat(u64),
    Float(f64),
    Boolean(bool),
    Text(String),
    Bytes(Vec<u64>),
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
    Lam(Box<ABT<Term>>, Vec<(Symbol, usize, usize)>),
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

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum ABT<Content> {
    Var(Symbol, usize), // usage number
    Cycle(Box<ABT<Content>>),
    // number of usages expected
    Abs(Symbol, usize, Box<ABT<Content>>),
    Tm(Content),
}

#[derive(
    Serialize, Deserialize, Debug, Clone, std::cmp::Eq, std::hash::Hash, std::cmp::PartialEq,
)]
pub struct NameSegment {
    pub text: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modifier {
    Structural,
    Unique(String),
    Opaque,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TypeDecl {
    Effect(DataDecl),
    Data(DataDecl),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataDecl {
    pub modifier: Modifier,
    pub bound: Vec<Symbol>,
    pub constructors: Vec<(Symbol, ABT<Type>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Causal<Contents> {
    One(Contents),
    Cons(Hash, Contents),
    Merge(Vec<Hash>, Contents),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawBranch {
    pub terms: Star<Referent, NameSegment>,
    pub types: Star<Reference, NameSegment>,
    pub children: HashMap<NameSegment, Hash>,
    pub edits: HashMap<NameSegment, Hash>,
}

impl RawBranch {
    pub fn merge(&mut self, other: &RawBranch) {
        self.terms.merge(&other.terms);
        self.types.merge(&other.types);
        self.children.extend(other.children.clone());
        self.edits.extend(other.edits.clone());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub raw: RawBranch,
    pub children: HashMap<NameSegment, Branch>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Star<K: std::hash::Hash + std::cmp::Eq + Clone, V> {
    pub fact: std::collections::HashSet<K>,
    pub d1: HashMap<K, V>,
    pub d2: HashMap<K, Reference>,
    pub d3: HashMap<K, (Reference, Reference)>,
}

impl<K: std::hash::Hash + std::cmp::Eq + Clone, V: Clone> Star<K, V> {
    fn merge(&mut self, other: &Self) {
        self.fact.extend(other.fact.clone());
        self.d1.extend(other.d1.clone());
        self.d2.extend(other.d2.clone());
        self.d3.extend(other.d3.clone());
    }
}

impl ABT<Term> {
    fn to_term(self) -> Option<Term> {
        match self {
            ABT::Tm(t) => Some(t),
            _ => None,
        }
    }
}

pub struct GC {
    values: Vec<Value>,
}

impl GC {
    pub fn new() -> GC {
        GC { values: vec![] }
    }
    pub fn put(&mut self, v: Value) -> usize {
        // match v {
        //     Value::GC(v) => v,
        //     _ => {
        self.values.push(v);
        self.values.len() - 1
        //     }
        // }
    }
    pub fn get(&self, n: usize) -> &Value {
        &self.values[n]
    }
    // pub fn pop(&mut self, n: usize) -> Value {
    //     self.values.remove(n)
    // }
}

impl Into<Value> for Term {
    fn into(self) -> Value {
        match self {
            Term::Int(i) => Value::Int(i),
            Term::Nat(a) => Value::Nat(a),
            Term::Float(a) => Value::Float(a),
            Term::Boolean(a) => Value::Boolean(a),
            Term::Text(a) => Value::Text(a),
            Term::Bytes(a) => Value::Bytes(a),
            Term::Char(a) => Value::Char(a),
            Term::Ref(a) => Value::Ref(a),
            Term::Constructor(a, b) => Value::Constructor(a, b),
            Term::Request(a, b) => Value::Request(a, b),
            // Term::Sequence(a) => Value::Sequence(
            //     a.into_iter()
            //         .map(|x| Rc::new(x.to_term().unwrap().into_value(gc)))
            //         .collect(),
            // ),
            Term::TermLink(a) => Value::TermLink(a),
            Term::TypeLink(a) => Value::TypeLink(a),
            _ => unreachable!("Cannot convert to a value"),
        }
    }
}
