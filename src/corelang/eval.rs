use super::syntax::{equiv_term, substitution, Term};
use crate::common::fileinfo::CompileError;
use num_traits::Zero;

type EvalResult = Result<Term, CompileError>;

pub fn eval(term: &Term) -> EvalResult {
    match term {
        Term::Apply(_, t1, ts) => {
            let e1 = eval(&(**t1))?;
            if let Term::Eq(info) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    return Ok(Term::Bool(None, equiv_term(&e2, &e3)));
                }
                Err(CompileError {
                    info,
                    message: None, // TODO
                })
            } else if let Term::Eval(info) = e1 {
                if ts.len() == 1 {
                    let e2 = eval(&ts[0])?;
                    if let Term::Quote(_, quoted) = e2 {
                        return eval(&quoted);
                    }
                    return Err(CompileError {
                        info,
                        message: Some("quote expected".into()), // TODO
                    });
                }
                Err(CompileError {
                    info,
                    message: Some("eval error".into()), // TODO
                })
            } else if let Term::If(info) = e1 {
                if ts.len() == 3 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    let e4 = eval(&ts[2])?;
                    match e2 {
                        Term::Bool(_, true) => return Ok(e3),
                        Term::Bool(_, false) => return Ok(e4),
                        _ => {
                            return Err(CompileError {
                                info: None,                                                // TODO
                                message: Some("If condition clause expects bool.".into()), // TODO
                            });
                        }
                    };
                }
                Err(CompileError {
                    info,
                    message: None, // TODO
                })
            } else if let Term::Add(info) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    if let Term::Number(_, n2) = e2 {
                        if let Term::Number(_, n3) = e3 {
                            return Ok(Term::Number(None, n2 + n3));
                        }
                    }
                }
                Err(CompileError {
                    info,
                    message: Some("Addition operator only accepts 2 numbers.".into()),
                })
            } else if let Term::Sub(info) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
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
                Err(CompileError {
                    info,
                    message: Some("Subtraction operator only accepts 2 numbers.".into()),
                })
            } else if let Term::Car(info) = e1 {
                if ts.len() == 1 {
                    let e2 = eval(&ts[0])?;
                    if let Term::Cons(_, t1, _) = e2 {
                        return Ok(*t1);
                    }
                }
                Err(CompileError {
                    info,
                    message: Some("Car operator only 1 cons.".into()),
                })
            } else if let Term::Cdr(info) = e1 {
                if ts.len() == 1 {
                    let e2 = eval(&ts[0])?;
                    if let Term::Cons(_, _, t2) = e2 {
                        return Ok(*t2);
                    }
                }
                Err(CompileError {
                    info,
                    message: Some("Cdr operator only 1 cons.".into()),
                })
            } else if let Term::Lambda(info, arg_num, body) = e1 {
                if ts.len() == arg_num {
                    return eval(&substitution(&*body, 0, ts));
                }
                Err(CompileError {
                    info,
                    message: Some(format!(
                        "The lambda function need {} args but got {} args.",
                        arg_num,
                        ts.len()
                    )),
                })
            } else {
                Err(CompileError {
                    info: None, // TODO
                    message: Some("Operator expected.".into()),
                })
            }
        }
        _ => Ok((*term).map_file_info(|_info| None)),
    }
}
