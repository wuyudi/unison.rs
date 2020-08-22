use std::collections::HashMap;

use super::env::*;
use super::parser;
use super::types::*;

// trait Eval
/*

Ok, what's the simplest way of doing this that I can imagine?

The thing that seems like it'll be hardest to do is the Effect system.
Because we need multi-shot continuations.
Which means I think I need stack frames.

but first, I feel like it makes sense to just try to get a bare-bones something working,
and then I can think about stack frames. Right?

after all, the stack is a Vec of Map's, right? Mapping from variable names to values?
hrmmm but not completely, because I also need to represent the exact place where we'll be returning to.
And being able to "pick up where we left off".

What does this mean?

So, one way to think of "where are we going" is to assign each "moment" in a given term an index ...
... and say "this is where we're going back to".
So like, .. the moments would be .. yeah, it would be a local stack of bindings, and an index of where we are at ..
and then "resuming" a continuation would mean going to the given term, walking the tree until we get to the index,
and resuming.
I like that.

*/
pub trait Eval {
    fn eval(&self, env: &mut Env, stack: &Stack) -> Term;
}

impl ABT<Term> {
    pub fn eval_with_bindings(&self, env: &mut Env, stack: &Stack, bindings: Vec<Term>) -> Term {
        // let kvs = vec![];
        let mut body = self;
        let mut new_stack = stack.clone();
        for binding in bindings.into_iter() {
            match body {
                ABT::Abs(sym, inner) => {
                    new_stack.set(sym.text.clone(), binding);
                    body = inner;
                }
                _ => unreachable!("NOP"),
            }
        }
        body.eval(env, &new_stack)
    }
}

fn unroll_cycle(
    inner: &ABT<Term>,
    names: &mut Vec<String>,
) -> (Vec<Box<ABT<Term>>>, Box<ABT<Term>>) {
    match inner {
        ABT::Abs(sym, inner) => {
            names.push(sym.text.clone());
            match &**inner {
                ABT::Tm(Term::LetRec(_, things, body)) => (things.clone(), body.clone()),
                _ => unroll_cycle(inner, names),
            }
        }
        _ => unreachable!("Cycle not abs"),
    }
}
/*

mnetal model for recursion-

you have a fn
thta fn has a scope that it came iwth
that scope needs to include itself
so, when you execute the fn
you first copy it into itself, and then go to town.

yeah sounds easy enough.

*/

impl Eval for ABT<Term> {
    fn eval(&self, env: &mut Env, stack: &Stack) -> Term {
        match self {
            ABT::Var(sym) => stack.lookup(&sym.text),
            ABT::Cycle(inner) => {
                println!("Here we have a cucle");
                let mut names = vec![];
                let (values, body) = unroll_cycle(inner, &mut names);
                let mut new_stack = stack.clone();
                // let
                for i in 0..names.len() {
                    let f = values[values.len() - 1 - i].eval(env, stack);
                    let f = match f {
                        Term::ScopedFunction(contents, term, bindings) => {
                            Term::CycleFunction(contents, term, bindings, names[i].clone())
                        }
                        _ => unreachable!("{:?}", f),
                    };
                    new_stack.set(names[i].clone(), f);
                }
                body.eval(env, &new_stack)
                // env.cycles.push()
                // match res {
                //     Term::ScopedFunction(contents, term, bindings) =>
                //         Term::CycleFunction(contents, term, bindings, num)
                // }
                // unreachable!("");
            }
            ABT::Abs(sym, inner) => unreachable!("Raw abs {}", stack.0[0].term),
            ABT::Tm(inner) => inner.eval(env, stack),
        }
    }
}

// Types of things we'll be dealing with:
// Terms as they are
// Partially applied function calls
// Builtins (which can be functions, mostly are probably?) - I guess a Ref() can be what we do for builtins

impl Eval for Term {
    fn eval(&self, env: &mut Env, stack: &Stack) -> Term {
        match self {
            Term::Int(_)
            | Term::Nat(_)
            | Term::Float(_)
            | Term::Boolean(_)
            | Term::Text(_)
            | Term::Char(_)
            | Term::TermLink(_)
            | Term::TypeLink(_)
            | Term::Blank => self.clone(),
            Term::Let(_, value, contents) => match &**contents {
                ABT::Abs(name, contents) => {
                    let value = value.eval(env, stack);
                    contents.eval(env, &stack.with(name.text.clone(), value))
                }
                _ => unreachable!(),
            },
            Term::Ref(Reference::DerivedId(Id(hash, _, _))) => env
                .load(&hash.to_string())
                .eval(env, &stack.with_frame(hash.to_string())),
            Term::Ref(Reference::Builtin(contents)) => {
                match contents.as_str() {
                    "Text.empty" => Term::Text("".to_string()),
                    "Sequence.empty" => Term::Sequence(vec![]),
                    // "Bytes.empty" => Term::BYtes(""),
                    _ => self.clone(),
                }
            }

            Term::Constructor(_, _) => self.clone(),
            Term::Request(_, _) => unimplemented!("Request {:?}", self),
            Term::Handle(contents, handler) => unimplemented!("Handle {:?}", self),
            Term::LetRec(_, values, body) => unimplemented!("LetRec {:?}", self),
            Term::Match(term, arms) => {
                let term = term.eval(env, stack);
                for MatchCase(pattern, where_term, body) in arms {
                    match pattern.matches(&term, where_term, env, &stack) {
                        None => (),
                        Some(bindings) => {
                            return (*body).eval_with_bindings(env, stack, bindings);
                        }
                    }
                }
                unreachable!("Nothing matched {:?}\n{:?}", term, arms);
            }

            Term::Ann(term, _type) => term.eval(env, stack),
            Term::Sequence(contents) => {
                let mut res = vec![];
                for item in contents.iter() {
                    res.push(Box::new(ABT::Tm(item.eval(env, stack))))
                }
                Term::Sequence(res)
            }
            Term::If(one, two, three) => match one.eval(env, stack) {
                Term::Boolean(true) => two.eval(env, stack),
                Term::Boolean(false) => three.eval(env, stack),
                _ => unreachable!("If with not a bool {:?}", self),
            },
            Term::And(one, two) => match (one.eval(env, stack), two.eval(env, stack)) {
                (Term::Boolean(a), Term::Boolean(b)) => Term::Boolean(a && b),
                (a, b) => unreachable!("and not bool {:?} and {:?}", a, b),
            },
            Term::Or(one, two) => match (one.eval(env, stack), two.eval(env, stack)) {
                (Term::Boolean(a), Term::Boolean(b)) => Term::Boolean(a || b),
                (a, b) => unreachable!("or not bool {:?} and {:?}", a, b),
            },
            // Term::Lam(contents) => Term::Lam(Box::new(ABT::Tm(contents.eval(env, stack)))),
            Term::Lam(contents) => Term::ScopedFunction(
                contents.clone(),
                stack.0[0].term.clone(),
                stack.0[0].bindings.clone(),
            ),
            // Term::LetRec(
            Term::App(one, two) => {
                let one = one.eval(env, stack);
                match one {
                    Term::Constructor(r, u) => {
                        Term::PartialConstructor(r, u, vec![two.eval(env, stack)])
                    }
                    Term::PartialConstructor(r, u, c) => {
                        let mut c = c.clone();
                        c.push(two.eval(env, stack));
                        Term::PartialConstructor(r, u, c)
                    }
                    Term::PartialNativeApp(name, body) => {
                        match (name.as_str(), body.as_slice(), &two.eval(env, stack)) {
                            ("Int.+", [Term::Int(a)], Term::Int(b)) => Term::Int(a + b),
                            ("Int.-", [Term::Int(a)], Term::Int(b)) => Term::Int(a - b),
                            ("Int.*", [Term::Int(a)], Term::Int(b)) => Term::Int(a * b),
                            ("Int./", [Term::Int(a)], Term::Int(b)) => Term::Int(a / b),
                            ("Int.<", [Term::Int(a)], Term::Int(b)) => Term::Boolean(*a < *b),
                            ("Int.<=", [Term::Int(a)], Term::Int(b)) => Term::Boolean(*a <= *b),
                            ("Int.>", [Term::Int(a)], Term::Int(b)) => Term::Boolean(*a > *b),
                            ("Int.>=", [Term::Int(a)], Term::Int(b)) => Term::Boolean(*a >= *b),
                            ("Int.==", [Term::Int(a)], Term::Int(b)) => Term::Boolean(*a == *b),
                            ("Int.and", [Term::Int(a)], Term::Int(b)) => Term::Int(a & b),
                            ("Int.or", [Term::Int(a)], Term::Int(b)) => Term::Int(a | b),
                            ("Int.xor", [Term::Int(a)], Term::Int(b)) => Term::Int(a ^ b),
                            ("Int.mod", [Term::Int(a)], Term::Int(b)) => Term::Int(a % b),
                            ("Int.pow", [Term::Int(a)], Term::Nat(b)) => {
                                Term::Int(a.pow(*b as u32))
                            }
                            ("Int.shiftLeft", [Term::Int(a)], Term::Nat(b)) => {
                                Term::Int(a >> *b as u32)
                            }
                            ("Int.shiftRight", [Term::Int(a)], Term::Nat(b)) => {
                                Term::Int(a << *b as u32)
                            }

                            ("Nat.+", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a + b),
                            ("Nat.*", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a * b),
                            ("Nat./", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a / b),
                            ("Nat.>", [Term::Nat(a)], Term::Nat(b)) => Term::Boolean(*a > *b),
                            ("Nat.>=", [Term::Nat(a)], Term::Nat(b)) => Term::Boolean(*a >= *b),
                            ("Nat.<", [Term::Nat(a)], Term::Nat(b)) => Term::Boolean(*a < *b),
                            ("Nat.<=", [Term::Nat(a)], Term::Nat(b)) => Term::Boolean(*a <= *b),
                            ("Nat.==", [Term::Nat(a)], Term::Nat(b)) => Term::Boolean(*a == *b),
                            ("Nat.and", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a & b),
                            ("Nat.or", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a | b),
                            ("Nat.xor", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a ^ b),
                            ("Nat.mod", [Term::Nat(a)], Term::Nat(b)) => Term::Nat(a % b),
                            ("Nat.pow", [Term::Nat(a)], Term::Nat(b)) => {
                                Term::Nat(a.pow(*b as u32))
                            }
                            ("Nat.shiftLeft", [Term::Nat(a)], Term::Nat(b)) => {
                                Term::Nat(a >> *b as u32)
                            }
                            ("Nat.shiftRight", [Term::Nat(a)], Term::Nat(b)) => {
                                Term::Nat(a << *b as u32)
                            }

                            // , ("Nat.drop", 2, DropN (Slot 1) (Slot 0))
                            // , ("Nat.sub", 2, SubN (Slot 1) (Slot 0))
                            // , ("Nat.mod", 2, ModN (Slot 1) (Slot 0))
                            // , ("Nat.pow", 2, PowN (Slot 1) (Slot 0))
                            ("Float.+", [Term::Float(a)], Term::Float(b)) => Term::Float(a + b),
                            ("Float.-", [Term::Float(a)], Term::Float(b)) => Term::Float(a - b),
                            ("Float.*", [Term::Float(a)], Term::Float(b)) => Term::Float(a * b),
                            ("Float./", [Term::Float(a)], Term::Float(b)) => Term::Float(a / b),
                            ("Float.<", [Term::Float(a)], Term::Float(b)) => Term::Boolean(*a < *b),
                            ("Float.<=", [Term::Float(a)], Term::Float(b)) => {
                                Term::Boolean(*a <= *b)
                            }
                            ("Float.>", [Term::Float(a)], Term::Float(b)) => Term::Boolean(*a > *b),
                            ("Float.>=", [Term::Float(a)], Term::Float(b)) => {
                                Term::Boolean(*a >= *b)
                            }
                            ("Float.==", [Term::Float(a)], Term::Float(b)) => {
                                Term::Boolean(*a == *b)
                            }

                            ("Universal.==", [one], two) => Term::Boolean(one == two),
                            ("Universal.>", [one], two) => Term::Boolean(one > two),
                            ("Universal.<", [one], two) => Term::Boolean(one < two),
                            ("Universal.>=", [one], two) => Term::Boolean(one >= two),
                            ("Universal.<=", [one], two) => Term::Boolean(one <= two),
                            ("Universal.compare", [one], two) => Term::Int(if one < two {
                                -1
                            } else if one > two {
                                1
                            } else {
                                0
                            }),

                            (a, b, c) => unreachable!(
                                "Native app, we dont have more than two args {} - {:?} - {:?}",
                                a, b, c
                            ),
                        }
                    }

                    Term::Ref(Reference::Builtin(builtin)) => {
                        match (builtin.as_str(), two.eval(env, stack)) {
                            ("Int.increment", Term::Int(i)) => Term::Int(i + 1),
                            ("Int.negate", Term::Int(i)) => Term::Int(-i),
                            ("Int.isEven", Term::Int(i)) => Term::Boolean(i % 2 == 0),
                            ("Int.isOdd", Term::Int(i)) => Term::Boolean(i % 2 == 1),
                            ("Nat.increment", Term::Nat(i)) => Term::Nat(i + 1),
                            ("Nat.isEvent", Term::Nat(i)) => Term::Boolean(i % 2 == 0),
                            ("Nat.isOdd", Term::Nat(i)) => Term::Boolean(i % 2 == 1),
                            ("Nat.toInt", Term::Nat(i)) => Term::Int(i as i64),
                            ("Boolean.not", Term::Boolean(i)) => Term::Boolean(!i),

                            (builtin, two) => Term::PartialNativeApp(builtin.to_owned(), vec![two]),
                        }
                    }
                    // Term::CycleFunction(names, )
                    Term::CycleFunction(contents, term, bindings, self_name) => match &*contents {
                        ABT::Abs(name, contents) => {
                            println!("Evaling a cycle: {} : {:?}", self_name, contents);
                            let two = two.eval(env, stack);
                            let mut inner_stack = stack.with_frame(term.clone());
                            for (k, v) in bindings.clone() {
                                inner_stack.set(k, v);
                            }
                            inner_stack.set(
                                self_name.clone(),
                                Term::CycleFunction(
                                    Box::new(ABT::Abs(name.clone(), contents.clone())),
                                    term,
                                    bindings,
                                    self_name,
                                ),
                            );
                            contents.eval(env, &inner_stack.with(name.text.clone(), two))
                        }
                        contents => unreachable!("Lam {:?}", contents),
                    },
                    Term::ScopedFunction(contents, term, bindings) => match &*contents {
                        ABT::Abs(name, contents) => {
                            println!("Evaling a fn {:?}", contents);
                            let two = two.eval(env, stack);
                            let mut inner_stack = stack.with_frame(term);
                            for (k, v) in bindings {
                                inner_stack.set(k, v);
                            }
                            contents.eval(env, &inner_stack.with(name.text.clone(), two))
                        }
                        contents => unreachable!("Lam {:?}", contents),
                    },
                    one => unreachable!("Apply top: {:?}", one),
                }
            }
            _ => unreachable!(),
        }
    }
}
