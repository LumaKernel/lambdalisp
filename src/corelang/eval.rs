use super::syntax::{equiv_term, substitution, Term};
use crate::common::fileinfo::CompileError;
use num_traits::Zero;

type EvalResult = Result<Term, CompileError>;

pub fn eval(term: &Term) -> EvalResult {
    match term {
        Term::Apply(info, t1, ts) => {
            let e1 = eval(&(**t1))?;
            if let Term::Eq(info_eq) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    return Ok(Term::Bool(None, equiv_term(&e2, &e3)));
                }
                Err(CompileError {
                    info: info_eq.or_else(|| info.clone()),
                    message: None, // TODO
                })
            } else if let Term::Eval(info_eval) = e1 {
                if ts.len() == 1 {
                    let e2 = eval(&ts[0])?;
                    if let Term::Quote(_, quoted) = e2 {
                        return eval(&quoted);
                    }
                    return Err(CompileError {
                        info: info_eval.or_else(|| info.clone()),
                        message: Some("quote expected".into()), // TODO
                    });
                }
                Err(CompileError {
                    info: info_eval.or_else(|| info.clone()),
                    message: Some("eval error".into()), // TODO
                })
            } else if let Term::Add(info_add) = e1 {
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
                    info: info_add.or_else(|| info.clone()),
                    message: Some("addition operator only accepts 2 numbers.".into()),
                })
            } else if let Term::Sub(info_sub) = e1 {
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
                    info: info_sub.or_else(|| info.clone()),
                    message: Some(format!(
                        "subtraction operator only accepts 2 numbers: found {} args",
                        ts.len()
                    )),
                })
            } else if let Term::Mul(info_mul) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    if let Term::Number(_, n2) = e2 {
                        if let Term::Number(_, n3) = e3 {
                            return Ok(Term::Number(None, n2 * n3));
                        }
                    }
                }
                Err(CompileError {
                    info: info_mul.or_else(|| info.clone()),
                    message: Some("multiplication operator only accepts 2 numbers.".into()),
                })
            } else if let Term::Div(info_div) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    if let Term::Number(_, n2) = e2 {
                        if let Term::Number(info_e3, n3) = e3 {
                            return if n3.is_zero() {
                                Err(CompileError {
                                    info: info_e3.or_else(|| info_div).or_else(|| info.clone()),
                                    message: Some("division operator got 0 for divisor.".into()),
                                })
                            } else {
                                Ok(Term::Number(None, n2 / n3))
                            };
                        }
                    }
                }
                Err(CompileError {
                    info: info_div.or_else(|| info.clone()),
                    message: Some("division operator only accepts 2 numbers.".into()),
                })
            } else if let Term::Rem(info_rem) = e1 {
                if ts.len() == 2 {
                    let e2 = eval(&ts[0])?;
                    let e3 = eval(&ts[1])?;
                    if let Term::Number(_, n2) = e2 {
                        if let Term::Number(info_e3, n3) = e3 {
                            return if n3.is_zero() {
                                Err(CompileError {
                                    info: info_e3.or_else(|| info_rem).or_else(|| info.clone()),
                                    message: Some("remainder operator got 0 for divisor.".into()),
                                })
                            } else {
                                Ok(Term::Number(None, n2 % n3))
                            };
                        }
                    }
                }
                Err(CompileError {
                    info: info_rem.or_else(|| info.clone()),
                    message: Some("remainder operator only accepts 2 numbers.".into()),
                })
            } else if let Term::Car(info_car) = e1 {
                if ts.len() == 1 {
                    let e2 = eval(&ts[0])?;
                    if let Term::Cons(_, t1, _) = e2 {
                        return Ok(*t1);
                    }
                }
                Err(CompileError {
                    info: info_car.or_else(|| info.clone()),
                    message: Some("car operator only 1 cons.".into()),
                })
            } else if let Term::Cdr(info_cdr) = e1 {
                if ts.len() == 1 {
                    let e2 = eval(&ts[0])?;
                    if let Term::Cons(_, _, t2) = e2 {
                        return Ok(*t2);
                    }
                }
                Err(CompileError {
                    info: info_cdr.or_else(|| info.clone()),
                    message: Some("cdr operator only 1 cons.".into()),
                })
            } else if let Term::Lambda(info_lambda, arg_num, body) = e1 {
                if ts.len() == arg_num {
                    return eval(&substitution(&*body, 0, ts));
                }
                Err(CompileError {
                    info: info_lambda.or_else(|| info.clone()),
                    message: Some(format!(
                        "the lambda function needs {} args but found {} arg(s)",
                        arg_num,
                        ts.len()
                    )),
                })
            } else {
                Err(CompileError {
                    info: info.clone(),
                    message: Some("operator expected".into()),
                })
            }
        }
        Term::If(info, t1, t2, t3) => {
            let e1 = eval(&t1)?;
            match e1 {
                Term::Bool(_, true) => return Ok(eval(&t2)?),
                Term::Bool(_, false) => return Ok(eval(&t3)?),
                _ => {
                    return Err(CompileError {
                        info: e1.file_info().clone().or_else(|| info.clone()),
                        message: Some("expect bool for if condition".into()),
                    });
                }
            };
        }
        _ => Ok((*term).map_file_info(|_info| None)),
    }
}
