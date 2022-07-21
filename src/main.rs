mod base {
    #[derive(Clone, Debug)]
    pub struct Location {
        /// filepath
        pub filepath: String,
        /// 0-based line number
        pub line: usize,
        /// 0-based column number
        pub col: usize,
    }
    #[derive(Clone, Debug)]
    pub struct Range {
        /// range from inclusive
        pub from: Location,
        /// range to inclusive
        pub to: Location,
    }
    #[derive(Clone, Debug)]
    pub struct FileInfo {
        pub name: String,
        pub range: Range,
    }
}

pub mod core {
    use super::base::FileInfo;
    use num_bigint::BigUint;
    use num_traits::Zero;

    // syntax
    #[derive(Clone, Debug)]
    pub enum Term {
        /// (operator, operand)
        Apply(Option<FileInfo>, Box<Term>, Vec<Term>),
        /// (number of args, body)
        Lambda(Option<FileInfo>, usize, Box<Term>),

        Quote(Option<FileInfo>, Box<Term>),
        /// (De Bruijn indexed, arg index in lambda)
        Variable(Option<FileInfo>, usize, usize),
        Eq(Option<FileInfo>),

        // list
        Cons(Option<FileInfo>, Box<Term>, Box<Term>),
        Nil(Option<FileInfo>),

        // arith
        Number(Option<FileInfo>, BigUint),

        // bool
        Bool(Option<FileInfo>, bool),

        // arith op
        Add(Option<FileInfo>),
        Sub(Option<FileInfo>),
        Mul(Option<FileInfo>),
        Div(Option<FileInfo>),
        Rem(Option<FileInfo>),

        // bool op
        If(Option<FileInfo>),

        // list op
        Head(Option<FileInfo>),
        Tail(Option<FileInfo>),
    }

    impl Term {
        pub fn map_file_info<F>(&self, f: F) -> Self
        where
            F: FnOnce(Option<FileInfo>) -> Option<FileInfo>,
        {
            match self {
                Term::Apply(info, t1, ts) => {
                    Term::Apply(f((*info).clone()), (*t1).clone(), (*ts).clone())
                }
                Term::Lambda(info, a1, t1) => Term::Lambda(f((*info).clone()), *a1, (*t1).clone()),

                Term::Quote(info, t1) => Term::Quote(f((*info).clone()), (*t1).clone()),
                Term::Variable(info, v1, a1) => Term::Variable(f((*info).clone()), *v1, *a1),

                Term::Cons(info, t1, t2) => {
                    Term::Cons(f((*info).clone()), (*t1).clone(), (*t2).clone())
                }

                Term::Nil(info) => Term::Nil(f((*info).clone())),

                // arith atom
                Term::Number(info, n1) => Term::Number(f((*info).clone()), (*n1).clone()),

                // bool atom
                Term::Bool(info, b1) => Term::Bool(f((*info).clone()), *b1),

                // bool op
                Term::If(info) => Term::If(f((*info).clone())),

                // arith op
                Term::Add(info) => Term::Add(f((*info).clone())),
                Term::Sub(info) => Term::Sub(f((*info).clone())),
                Term::Mul(info) => Term::Mul(f((*info).clone())),
                Term::Div(info) => Term::Div(f((*info).clone())),
                Term::Rem(info) => Term::Rem(f((*info).clone())),
                Term::Eq(info) => Term::Eq(f((*info).clone())),

                // list op
                Term::Head(info) => Term::Head(f((*info).clone())),
                Term::Tail(info) => Term::Tail(f((*info).clone())),
            }
        }

        pub fn map_subterm<F>(&self, f: F) -> Self
        where
            F: Fn(Self) -> Self,
        {
            match self {
                Term::Apply(info, t1, ts) => Term::Apply(
                    (*info).clone(),
                    f((**t1).clone()).into(),
                    (*ts).clone().into_iter().map(f).collect(),
                ),
                Term::Lambda(info, a1, t1) => {
                    Term::Lambda((*info).clone(), *a1, f((**t1).clone()).into())
                }

                Term::Quote(info, t1) => Term::Quote((*info).clone(), f((**t1).clone()).into()),
                Term::Variable(info, v1, a1) => Term::Variable((*info).clone(), *v1, *a1),

                Term::Cons(info, t1, t2) => Term::Cons(
                    (*info).clone(),
                    f((**t1).clone()).into(),
                    f((**t2).clone()).into(),
                ),

                _ => self.clone(),
            }
        }
    }

    /// Shift De Bruijn index by d more than or equal to c.
    /// c: threshold inclusive
    /// d: increased width
    pub fn shift_index(t: Term, c: usize, d: usize) -> Term {
        match t {
            Term::Variable(info, v, a) => Term::Variable(info, if v < c { v } else { v + d }, a),
            Term::Lambda(info, arg_num, body) => {
                Term::Lambda(info, arg_num, shift_index(*body, c + 1, d).into())
            }

            _ => t.map_subterm(|s| shift_index(s, c, d)),
        }
    }

    pub fn equiv_term_vec(t1: Vec<Term>, t2: Vec<Term>) -> bool {
        use std::iter::zip;
        if t1.len() != t2.len() {
            return false;
        }
        !zip(t1, t2).any(|(e1, e2)| !equiv_term(e1, e2))
    }

    pub fn equiv_term(t1: Term, t2: Term) -> bool {
        match t1 {
            Term::Apply(_, t11, ts1) => {
                if let Term::Apply(_, t21, ts2) = t2 {
                    return equiv_term(*t11, *t21) && equiv_term_vec(ts1, ts2);
                }
            }
            Term::Lambda(_, a1, t11) => {
                if let Term::Lambda(_, a2, t21) = t2 {
                    return a1 == a2 && equiv_term(*t11, *t21);
                }
            }

            Term::Quote(_, t11) => {
                if let Term::Quote(_, t21) = t2 {
                    return equiv_term(*t11, *t21);
                }
            }
            Term::Variable(_, v1, a1) => {
                if let Term::Variable(_, v2, a2) = t2 {
                    return v1 == v2 && a1 == a2;
                }
            }

            Term::Cons(_, t11, t12) => {
                if let Term::Cons(_, t21, t22) = t2 {
                    return equiv_term(*t11, *t21) && equiv_term(*t12, *t22);
                }
            }

            Term::Nil(_) => {
                if let Term::Nil(_) = t2 {
                    return true;
                }
            }

            // arith atom
            Term::Number(_, n1) => {
                if let Term::Number(_, n2) = t2 {
                    return n1 == n2;
                }
            }

            // bool atom
            Term::Bool(_, b1) => {
                if let Term::Bool(_, b2) = t2 {
                    return b1 == b2;
                }
            }

            // bool op
            Term::If(_) => {
                if let Term::If(_) = t2 {
                    return true;
                }
            }

            // arith op
            Term::Add(_) => {
                if let Term::Add(_) = t2 {
                    return true;
                }
            }
            Term::Sub(_) => {
                if let Term::Sub(_) = t2 {
                    return true;
                }
            }
            Term::Mul(_) => {
                if let Term::Mul(_) = t2 {
                    return true;
                }
            }
            Term::Div(_) => {
                if let Term::Div(_) = t2 {
                    return true;
                }
            }
            Term::Rem(_) => {
                if let Term::Rem(_) = t2 {
                    return true;
                }
            }
            Term::Eq(_) => {
                if let Term::Eq(_) = t2 {
                    return true;
                }
            }

            // list op
            Term::Head(_) => {
                if let Term::Head(_) = t2 {
                    return true;
                }
            }
            Term::Tail(_) => {
                if let Term::Tail(_) = t2 {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_value_term(term: Term) -> bool {
        match &term {
            Term::Cons(_, t1, t2) => is_value_term((**t1).clone()) && is_value_term((**t2).clone()),
            Term::Apply(..) => false,
            _ => true,
        }
    }

    pub fn substitution(term: Term, from: usize, to_vec: Vec<Term>) -> Term {
        match term {
            Term::Variable(_, v1, w1) if v1 == from => to_vec[w1].clone(),
            Term::Lambda(info, arg_num, body) => Term::Lambda(
                info,
                arg_num,
                substitution(
                    *body,
                    from + 1,
                    to_vec.into_iter().map(|e| shift_index(e, 0, 1)).collect(),
                )
                .into(),
            ),

            _ => term.map_subterm(|s| substitution(s, from, to_vec.clone())),
        }
    }

    // semantic

    pub struct EvalError {
        pub range: Option<FileInfo>,
        pub message: Option<String>,
    }
    type EvalResult = Result<Term, EvalError>;

    pub fn eval(term: Term) -> EvalResult {
        match term {
            Term::Apply(_, t1, ts) => {
                let e1 = eval(*t1.clone())?;
                if let Term::Eq(info) = e1 {
                    if ts.len() == 2 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        return Ok(Term::Bool(None, equiv_term(e2, e3)));
                    }
                    Err(EvalError {
                        range: info,
                        message: None, // TODO
                    })
                } else if let Term::If(info) = e1 {
                    if ts.len() == 3 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        let e4 = eval(ts[2].clone())?;
                        match e2 {
                            Term::Bool(_, true) => return Ok(e3),
                            Term::Bool(_, false) => return Ok(e4),
                            _ => {
                                return Err(EvalError {
                                    range: None,                                               // TODO
                                    message: Some("If conditino clause expects bool.".into()), // TODO
                                });
                            }
                        };
                    }
                    Err(EvalError {
                        range: info,
                        message: None, // TODO
                    })
                } else if let Term::Add(info) = e1 {
                    if ts.len() == 2 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        if let Term::Number(_, n2) = e2 {
                            if let Term::Number(_, n3) = e3 {
                                return Ok(Term::Number(None, n2 + n3));
                            }
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Addition operator only accepts 2 numbers.".into()),
                    })
                } else if let Term::Sub(info) = e1 {
                    if ts.len() == 2 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        if let Term::Number(_, n2) = e2 {
                            if let Term::Number(_, n3) = e3 {
                                return Ok(if n2 < n3 {
                                    Term::Number(None, Zero::zero())
                                } else {
                                    Term::Number(None, n2 - n3)
                                });
                            }
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Subtraction operator only accepts 2 numbers.".into()),
                    })
                } else if let Term::Mul(info) = e1 {
                    if ts.len() == 2 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        if let Term::Number(_, n2) = e2 {
                            if let Term::Number(_, n3) = e3 {
                                return Ok(Term::Number(None, n2 * n3));
                            }
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Multiplication operator only accepts 2 numbers.".into()),
                    })
                } else if let Term::Div(info) = e1 {
                    if ts.len() == 2 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        if let Term::Number(_, n2) = e2 {
                            if let Term::Number(_, n3) = e3 {
                                return Ok(Term::Number(None, n2 / n3));
                            }
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Division operator only accepts 2 numbers.".into()),
                    })
                } else if let Term::Rem(info) = e1 {
                    if ts.len() == 2 {
                        let e2 = eval(ts[0].clone())?;
                        let e3 = eval(ts[1].clone())?;
                        if let Term::Number(_, n2) = e2 {
                            if let Term::Number(_, n3) = e3 {
                                return Ok(Term::Number(None, n2 % n3));
                            }
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Reminder operator only accepts 2 numbers.".into()),
                    })
                } else if let Term::Head(info) = e1 {
                    if ts.len() == 1 {
                        let e2 = eval(ts[0].clone())?;
                        if let Term::Cons(_, head, _) = e2 {
                            return Ok(*head);
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Head operator only 1 list.".into()),
                    })
                } else if let Term::Tail(info) = e1 {
                    if ts.len() == 1 {
                        let e2 = eval(ts[0].clone())?;
                        if let Term::Cons(_, head, _) = e2 {
                            return Ok(*head);
                        }
                    }
                    Err(EvalError {
                        range: info,
                        message: Some("Head operator only 1 list.".into()),
                    })
                } else if let Term::Lambda(info, arg_num, body) = e1 {
                    if ts.len() == arg_num {
                        return eval(substitution(*body, 0, ts));
                    }
                    Err(EvalError {
                        range: info,
                        message: Some(format!(
                            "The lambda function need {} args but got {} args.",
                            arg_num,
                            ts.len()
                        )),
                    })
                } else {
                    Err(EvalError {
                        range: None, // TODO
                        message: Some("Operator expected.".into()),
                    })
                }
            }
            _ => Ok(term),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
