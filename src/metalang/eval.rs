use super::syntax::{transform_to_core, MetaEnv, MetaStatement};
use crate::common::fileinfo::CompileError;
use crate::corelang::eval::eval as core_eval;
use crate::corelang::syntax::Term as CoreTerm;

/// (new env, evaluated value)
pub fn eval(
    env: &MetaEnv,
    stmt: &MetaStatement,
) -> Result<(MetaEnv, Option<CoreTerm>), CompileError> {
    match stmt {
        MetaStatement::Def(_, def) => {
            let mut new_env = env.clone();
            new_env.insert(def.name.clone(), def.term.clone());
            Ok((new_env, None))
        }
        MetaStatement::DefRec(_, _dr) => Err(CompileError {
            info: None,
            message: Some("TODO: not implemented yet".into()),
        }),
        MetaStatement::Term(_, mt) => {
            Ok((env.clone(), Some(core_eval(&transform_to_core(env, mt)?)?)))
        }
    }
}

pub fn eval_all(
    env: &MetaEnv,
    stmts: &Vec<MetaStatement>,
) -> Result<(MetaEnv, Vec<CoreTerm>), CompileError> {
    let mut cur_env = env.clone();
    let mut values = Vec::<CoreTerm>::new();
    for stmt in stmts {
        let (new_env, value) = eval(env, stmt)?;
        cur_env = new_env;
        if let Some(v) = value {
            values.push(v);
        }
    }
    Ok((cur_env, values))
}
