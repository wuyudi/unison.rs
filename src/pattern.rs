use super::env::*;
use super::types::*;

impl Pattern {
    pub fn match_(&self, term: &Term) -> Option<Vec<Term>> {
        match (self, term) {
            (Pattern::EffectPure(_), Term::RequestWithContinuation(_, _, _, _, _)) => None,
            (Pattern::EffectPure(pattern), Term::RequestPure(inner)) => pattern.match_(inner),
            (
                Pattern::EffectBind(reference, number, args, kont),
                Term::RequestWithContinuation(tref, tnum, targs, tidx, tkont),
            ) if reference == tref && number == tnum && args.len() == targs.len() => {
                let mut all = vec![];
                for i in 0..args.len() {
                    match args[i].match_(&targs[i]) {
                        None => return None,
                        Some(inner) => {
                            all.extend(inner);
                        }
                    }
                }
                match &**kont {
                    Pattern::Unbound => (),
                    Pattern::Var => all.push(Term::Continuation(*tidx, tkont.clone())),
                    _ => unreachable!("Can't match on a continuation"),
                }
                Some(all)
            }
            (Pattern::Unbound, _) => Some(vec![]),
            (Pattern::Var, t) => Some(vec![t.clone()]),
            (Pattern::Boolean(a), Term::Boolean(b)) if a == b => Some(vec![]),
            (Pattern::Int(a), Term::Int(b)) if a == b => Some(vec![]),
            (Pattern::Nat(a), Term::Nat(b)) if a == b => Some(vec![]),
            (Pattern::Float(a), Term::Float(b)) if a == b => Some(vec![]),
            (Pattern::Text(a), Term::Text(b)) if a == b => Some(vec![]),
            (Pattern::Char(a), Term::Char(b)) if a == b => Some(vec![]),
            (Pattern::As(a), term) => match a.match_(term) {
                None => None,
                Some(mut terms) => {
                    terms.insert(0, term.clone());
                    Some(terms)
                }
            },
            (Pattern::SequenceLiteral(patterns), Term::Sequence(items))
                if patterns.len() == items.len() =>
            {
                let mut all = vec![];
                for i in 0..patterns.len() {
                    match &*items[i] {
                        ABT::Tm(term) => match patterns[i].match_(term) {
                            None => return None,
                            Some(v) => {
                                all.extend(v);
                            }
                        },
                        _ => unreachable!("Nonevaluated sequence item"),
                    }
                }
                return Some(all);
            }
            (Pattern::SequenceOp(one, op, two), Term::Sequence(contents)) => match op {
                SeqOp::Cons => {
                    if contents.len() > 0 {
                        match &*contents[0] {
                            ABT::Tm(term) => match one.match_(term) {
                                None => None,
                                Some(mut ones) => {
                                    match two.match_(&Term::Sequence(contents[1..].to_vec())) {
                                        None => None,
                                        Some(twos) => {
                                            ones.extend(twos);
                                            Some(ones)
                                        }
                                    }
                                }
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                SeqOp::Snoc => {
                    if contents.len() > 0 {
                        match &*contents[contents.len() - 1] {
                            ABT::Tm(term) => match one
                                .match_(&Term::Sequence(contents[..contents.len() - 1].to_vec()))
                            {
                                None => None,
                                Some(mut ones) => match two.match_(term) {
                                    None => None,
                                    Some(twos) => {
                                        ones.extend(twos);
                                        Some(ones)
                                    }
                                },
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                SeqOp::Concat => match (&**one, &**two) {
                    (Pattern::SequenceLiteral(patterns), two) => {
                        if contents.len() >= patterns.len() {
                            match one.match_(&Term::Sequence(contents[0..patterns.len()].to_vec()))
                            {
                                None => None,
                                Some(mut ones) => {
                                    match two.match_(&Term::Sequence(
                                        contents[patterns.len()..].to_vec(),
                                    )) {
                                        None => None,
                                        Some(twos) => {
                                            ones.extend(twos);
                                            Some(ones)
                                        }
                                    }
                                }
                            }
                        } else {
                            None
                        }
                    }
                    (_, Pattern::SequenceLiteral(patterns)) => {
                        if contents.len() >= patterns.len() {
                            let split = contents.len() - patterns.len();
                            match one.match_(&Term::Sequence(contents[0..split].to_vec())) {
                                None => None,
                                Some(mut ones) => {
                                    match two.match_(&Term::Sequence(contents[split..].to_vec())) {
                                        None => None,
                                        Some(twos) => {
                                            ones.extend(twos);
                                            Some(ones)
                                        }
                                    }
                                }
                            }
                        } else {
                            None
                        }
                    }
                    _ => unreachable!(),
                },
            },
            (Pattern::Constructor(reference, number, children), inner) => {
                let mut all = vec![];
                if children.len() > 0 {
                    match inner {
                        Term::PartialConstructor(r, n, pchildren)
                            if r == reference && n == number =>
                        {
                            if pchildren.len() != children.len() {
                                return None;
                            }
                            for i in 0..children.len() {
                                match children[i].match_(&pchildren[i]) {
                                    None => return None,
                                    Some(v) => {
                                        all.extend(v);
                                    }
                                }
                            }
                            return Some(all);
                        }
                        _ => return None,
                    }
                }

                match inner {
                    Term::Constructor(r, n) if r == reference && n == n => Some(all),
                    _ => None,
                }
                //
            }
            _ => None,
        }
    }
    pub fn matches(
        &self,
        term: &Term,
        where_term: &Option<Box<ABT<Term>>>,
        env: &mut Env,
        stack: &Stack,
    ) -> Option<Vec<Term>> {
        let bindings = self.match_(term);
        match bindings {
            None => None,
            Some(bindings) => match where_term {
                None => Some(bindings),
                Some(inner) => match inner.eval_with_bindings(env, stack, bindings.clone()) {
                    Term::Boolean(true) => Some(bindings),
                    _ => None,
                },
            },
        }
    }
}
