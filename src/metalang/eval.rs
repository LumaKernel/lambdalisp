use super::syntax::{transform_to_core, MetaEnv, MetaExport, MetaStatement, MetaTerm};
use crate::common::fileinfo::CompileError;
use crate::common::resolver::ContentResolver;
use crate::corelang::eval::eval as core_eval;
use crate::corelang::syntax::Term as CoreTerm;
use crate::metalang::parser::MetaParser;
use crate::resolver::lib::LibResolver;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

pub struct MetaEvaluator {
    pub env: MetaEnv,
    pub exported: MetaEnv,
    pub filepath: Option<PathBuf>,
    pub do_assert: bool,
    pub resolver: Rc<RefCell<Box<dyn ContentResolver>>>,
}

impl Default for MetaEvaluator {
    fn default() -> Self {
        Self {
            env: MetaEnv::default(),
            exported: MetaEnv::default(),
            filepath: None,
            do_assert: false,
            resolver: Rc::new(RefCell::new(Box::new(LibResolver::default()))),
        }
    }
}

fn substitution_rec(mt: &MetaTerm, name_vec: &Vec<&str>, name_set: &HashSet<&str>) -> MetaTerm {
    match mt {
        MetaTerm::Apply(info, t, ts) => {
            if let MetaTerm::Variable(_, v) = &**t {
                if name_set.contains(v.as_str()) {
                    let mut new_arg_vec: Vec<MetaTerm> = name_vec
                        .iter()
                        .map(|name| MetaTerm::Variable(None, name.to_string()))
                        .collect();
                    new_arg_vec.append(&mut ts.clone());
                    return MetaTerm::Apply(info.clone(), t.clone(), new_arg_vec);
                }
            }
        }
        _ => {}
    }
    mt.map_subterm(|st| substitution_rec(&st, name_vec, name_set))
}

impl MetaEvaluator {
    pub fn set_resolver(&mut self, resolver: Box<dyn ContentResolver>) {
        self.resolver = Rc::new(RefCell::new(resolver));
    }

    fn import(&mut self, to_resolve: String) -> Result<MetaEnv, CompileError> {
        let c = match (*self.resolver.borrow_mut()).resolve(&self.filepath, &to_resolve) {
            Ok(c) => c,
            Err(e) => Err(CompileError {
                info: None,
                message: Some(format!("resolve error: {}", e)),
            })?,
        };
        let mut p = MetaParser::new(to_resolve.clone(), c.content.to_string().chars().collect());
        let stmt_vec = p.parse_stmt_vec()?;
        let mut evaluator = MetaEvaluator {
            env: Default::default(),
            exported: Default::default(),
            filepath: c.filepath.clone(),
            do_assert: self.do_assert,
            resolver: self.resolver.clone(),
        };
        evaluator.eval_vec(&stmt_vec)?;
        Ok(evaluator.exported)
    }

    /// (new env, evaluated value)
    pub fn eval(&mut self, stmt: &MetaStatement) -> Result<Option<CoreTerm>, CompileError> {
        match stmt {
            MetaStatement::Def(_, name, term) => {
                self.env
                    .insert(name.clone(), transform_to_core(&self.env, &term)?);
                Ok(None)
            }
            MetaStatement::DefRec(info, fun_vec) => {
                let name_vec: Vec<&str> = fun_vec.iter().map(|fun| fun.name.as_str()).collect();
                let name_set: HashSet<&str> = fun_vec.iter().map(|fun| fun.name.as_str()).collect();
                let real_lambda_in_vec: Vec<_> = fun_vec
                    .clone()
                    .into_iter()
                    .map(|fun| {
                        let real_body = substitution_rec(&fun.term, &name_vec, &name_set);
                        let real_arg_name_vec = {
                            let mut new_vec: Vec<String> =
                                name_vec.clone().iter().map(|e| (*e).into()).collect();
                            new_vec.append(&mut fun.arg_name_vec.clone());
                            new_vec
                        };
                        let real_lambda_in = MetaTerm::Lambda(
                            info.clone(),
                            real_arg_name_vec.clone(),
                            real_body.into(),
                        );
                        (fun, real_lambda_in)
                    })
                    .collect();

                for (fun, real_lambda_in) in real_lambda_in_vec.iter() {
                    // (lambda (<orig_arg>) (real_lambda <real_lambda_vec> <orig_arg>))

                    // <real_lambda_vec> <orig_arg>
                    let real_operand = {
                        let mut real_operand: Vec<_> = real_lambda_in_vec
                            .iter()
                            .map(|(_, real_lambda_in)| real_lambda_in.clone())
                            .collect();
                        real_operand.append(
                            &mut fun
                                .arg_name_vec
                                .clone()
                                .into_iter()
                                .map(|v| MetaTerm::Variable(info.clone(), v))
                                .collect(),
                        );
                        real_operand
                    };

                    let real_lambda = MetaTerm::Lambda(
                        info.clone(),
                        fun.arg_name_vec.clone(),
                        MetaTerm::Apply(info.clone(), real_lambda_in.clone().into(), real_operand)
                            .into(),
                    );
                    self.env.insert(
                        fun.name.clone(),
                        transform_to_core(&self.env, &real_lambda)?,
                    );
                }
                Ok(None)
            }
            MetaStatement::Term(_, mt) => Ok(Some(core_eval(&transform_to_core(&self.env, mt)?)?)),
            MetaStatement::Assert(info, mt) => {
                if self.do_assert {
                    let v = core_eval(&transform_to_core(&self.env, mt)?)?;
                    if let CoreTerm::Bool(_, true) = v {
                        Ok(None)
                    } else {
                        Err(CompileError {
                            info: info.clone(),
                            message: Some("assertion failed".into()),
                        })
                    }
                } else {
                    Ok(None)
                }
            }
            MetaStatement::Import(_, import) => {
                for path in import {
                    for exported in self.import(path.into())? {
                        self.env.insert(exported.0, exported.1);
                    }
                }
                Ok(None)
            }
            MetaStatement::Export(info, export) => {
                for export_inner in export {
                    match export_inner {
                        MetaExport::Var(v) => match self.env.get(v) {
                            Some(val) => {
                                self.exported.insert(v.clone(), val.clone());
                            }
                            None => {
                                return Err(CompileError {
                                    info: info.clone(),
                                    message: Some(format!(
                                        "variable \"{}\" cannot be exported. not found",
                                        v
                                    )),
                                });
                            }
                        },
                        MetaExport::Path(path) => {
                            for exported in self.import(path.into())? {
                                self.exported.insert(exported.0, exported.1);
                            }
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn eval_vec(
        &mut self,
        stmt_vec: &Vec<MetaStatement>,
    ) -> Result<Vec<CoreTerm>, CompileError> {
        let mut value_vec = Vec::<CoreTerm>::new();
        for stmt in stmt_vec {
            let value = self.eval(stmt)?;
            if let Some(v) = value {
                value_vec.push(v);
            }
        }
        Ok(value_vec)
    }
}
