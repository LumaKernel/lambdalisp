use crate::common::fileinfo::{CompileError, FileInfo};
use crate::corelang::syntax::Term;
use num_bigint::BigUint;
use std::collections::{HashMap, HashSet};

// syntax
#[derive(Clone, Debug)]
pub enum MetaTerm {
    /// (operator, operand)
    Apply(Option<FileInfo>, Box<MetaTerm>, Vec<MetaTerm>),
    /// (arg names, body)
    Lambda(Option<FileInfo>, Vec<String>, Box<MetaTerm>),

    Quote(Option<FileInfo>, Box<MetaTerm>),
    Variable(Option<FileInfo>, String),
    Eq(Option<FileInfo>),

    If(
        Option<FileInfo>,
        Box<MetaTerm>,
        Box<MetaTerm>,
        Box<MetaTerm>,
    ),

    // structure
    Cons(Option<FileInfo>, Box<MetaTerm>, Box<MetaTerm>),
    Nil(Option<FileInfo>),

    // arith
    Number(Option<FileInfo>, BigUint),

    // bool
    Bool(Option<FileInfo>, bool),

    // meta op
    Eval(Option<FileInfo>),

    // arith op
    Add(Option<FileInfo>),
    Sub(Option<FileInfo>),
    Mul(Option<FileInfo>),
    Div(Option<FileInfo>),
    Rem(Option<FileInfo>),

    // structure op
    Car(Option<FileInfo>),
    Cdr(Option<FileInfo>),

    // meta specific
    List(Option<FileInfo>, Vec<MetaTerm>),
    // TODO: let be in
}

impl MetaTerm {
    pub fn file_info(&self) -> &Option<FileInfo> {
        match self {
            MetaTerm::Apply(info, ..) => info,
            MetaTerm::Lambda(info, ..) => info,
            MetaTerm::Quote(info, ..) => info,
            MetaTerm::Variable(info, ..) => info,
            MetaTerm::Eq(info, ..) => info,
            MetaTerm::Cons(info, ..) => info,
            MetaTerm::Nil(info, ..) => info,
            MetaTerm::Number(info, ..) => info,
            MetaTerm::Bool(info, ..) => info,
            MetaTerm::Eval(info, ..) => info,
            MetaTerm::Add(info, ..) => info,
            MetaTerm::Sub(info, ..) => info,
            MetaTerm::Mul(info, ..) => info,
            MetaTerm::Div(info, ..) => info,
            MetaTerm::Rem(info, ..) => info,
            MetaTerm::If(info, ..) => info,
            MetaTerm::Car(info, ..) => info,
            MetaTerm::Cdr(info, ..) => info,
            MetaTerm::List(info, ..) => info,
        }
    }

    pub fn map_subterm<F>(&self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        match self {
            Self::Apply(info, t1, ts) => Self::Apply(
                (*info).clone(),
                f((**t1).clone()).into(),
                (*ts).clone().into_iter().map(f).collect(),
            ),
            Self::Lambda(info, a1, t1) => {
                Self::Lambda((*info).clone(), a1.clone(), f((**t1).clone()).into())
            }

            Self::If(info, t1, t2, t3) => Self::If(
                (*info).clone(),
                f((**t1).clone()).into(),
                f((**t2).clone()).into(),
                f((**t3).clone()).into(),
            ),

            Self::Quote(info, t1) => Self::Quote((*info).clone(), f((**t1).clone()).into()),

            Self::Cons(info, t1, t2) => Self::Cons(
                (*info).clone(),
                f((**t1).clone()).into(),
                f((**t2).clone()).into(),
            ),

            Self::List(info, ts) => {
                Self::List((*info).clone(), (*ts).clone().into_iter().map(f).collect())
            }

            _ => self.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MetaStatement {
    /// (name, value)
    Def(Option<FileInfo>, String, MetaTerm),
    DefRec(Option<FileInfo>, Vec<DefRecFun>),
    Term(Option<FileInfo>, MetaTerm),
    Assert(Option<FileInfo>, MetaTerm),
    /// (path)
    Import(Option<FileInfo>, Vec<String>),
    /// (name)
    Export(Option<FileInfo>, Vec<MetaExport>),
}

impl MetaStatement {
    pub fn file_info(&self) -> &Option<FileInfo> {
        match self {
            MetaStatement::Def(info, _, _) => info,
            MetaStatement::DefRec(info, _) => info,
            MetaStatement::Term(info, _) => info,
            MetaStatement::Assert(info, _) => info,
            MetaStatement::Import(info, _) => info,
            MetaStatement::Export(info, _) => info,
        }
    }
}

#[derive(Clone, Debug)]
pub enum MetaExport {
    Var(String),
    Path(String),
}

#[derive(Clone, Debug)]
pub struct DefRecFun {
    pub name: String,
    pub arg_name_vec: Vec<String>,
    pub term: MetaTerm,
}

pub type MetaEnv = HashMap<String, Term>;

/// (De Bruijn index, arg index in lambda)
type ArgNameMap = HashMap<String, (usize, usize)>;
fn shift_arg_map(arg_map: ArgNameMap) -> ArgNameMap {
    arg_map
        .into_iter()
        .map(|(arg_name, (index, arg_num))| (arg_name, (index + 1, arg_num)))
        .collect()
}

fn transform_to_core_internal(
    env: &MetaEnv,
    mt: &MetaTerm,
    arg_map: &ArgNameMap,
) -> Result<Term, CompileError> {
    match mt {
        // Lambda and Variable are most important parts.
        MetaTerm::Lambda(info, args, body) => {
            let arg_set: HashSet<&String> = args.iter().collect();
            if arg_set.len() != args.len() {
                return Err(CompileError {
                    info: info.clone(),
                    message: Some("Variable names in one lambda should be unique.".into()),
                });
            }
            let mut sub_arg_map = shift_arg_map(arg_map.clone());
            for (arg_num, arg_name) in args.iter().enumerate() {
                sub_arg_map.insert(arg_name.clone(), (0, arg_num));
            }
            Ok(Term::Lambda(
                info.clone(),
                args.len(),
                transform_to_core_internal(env, body, &sub_arg_map)?.into(),
            ))
        }
        MetaTerm::Variable(info, var) => {
            match arg_map.get(var) {
                Some((v, a)) => Ok(Term::Variable(info.clone(), *v, *a)),
                None => {
                    // Env var which is not shadowed by local bound variables.
                    match env.get(var) {
                        Some(mt2) => Ok(mt2.clone()),
                        None => Err(CompileError {
                            info: info.clone(),
                            message: Some(format!(
                                "Variable name \"{}\" is not defined variable.",
                                var
                            )),
                        }),
                    }
                }
            }
        }

        MetaTerm::Apply(info, t1, ts) => Ok(Term::Apply(
            info.clone(),
            transform_to_core_internal(env, t1, arg_map)?.into(),
            {
                let cs = ts
                    .iter()
                    .map(|e| transform_to_core_internal(env, e, arg_map))
                    .collect::<Vec<_>>();

                if let Some(Err(err)) = cs.iter().find(|c| matches!(c, Err(..))) {
                    return Err(err.clone());
                }

                Ok(cs.into_iter().map(|c| c.unwrap()).collect())
            }?,
        )),

        MetaTerm::Quote(info, t) => Ok(Term::Quote(
            info.clone(),
            transform_to_core_internal(env, t, arg_map)?.into(),
        )),
        MetaTerm::Eq(info) => Ok(Term::Eq(info.clone())),

        MetaTerm::If(info, t1, t2, t3) => Ok(Term::If(
            info.clone(),
            transform_to_core_internal(env, t1, arg_map)?.into(),
            transform_to_core_internal(env, t2, arg_map)?.into(),
            transform_to_core_internal(env, t3, arg_map)?.into(),
        )),

        // structure
        MetaTerm::Cons(info, t1, t2) => Ok(Term::Cons(
            info.clone(),
            transform_to_core_internal(env, t1, arg_map)?.into(),
            transform_to_core_internal(env, t2, arg_map)?.into(),
        )),
        MetaTerm::Nil(info) => Ok(Term::Nil(info.clone())),

        // arith
        MetaTerm::Number(info, n) => Ok(Term::Number(info.clone(), (*n).clone())),

        // bool
        MetaTerm::Bool(info, b) => Ok(Term::Bool(info.clone(), *b)),

        // meta op
        MetaTerm::Eval(info) => Ok(Term::Eval(info.clone())),

        // arith op
        MetaTerm::Add(info) => Ok(Term::Add(info.clone())),
        MetaTerm::Sub(info) => Ok(Term::Sub(info.clone())),
        MetaTerm::Mul(info) => Ok(Term::Mul(info.clone())),
        MetaTerm::Div(info) => Ok(Term::Div(info.clone())),
        MetaTerm::Rem(info) => Ok(Term::Rem(info.clone())),

        // structure op
        MetaTerm::Car(info) => Ok(Term::Car(info.clone())),
        MetaTerm::Cdr(info) => Ok(Term::Cdr(info.clone())),

        // meta specific
        MetaTerm::List(info, vec) => Ok(transform_list_construction_to_core_list_internal(
            info.clone(),
            env,
            &*vec,
            arg_map,
        )?),
    }
}

fn transform_list_construction_to_core_list_internal(
    info: Option<FileInfo>,
    env: &MetaEnv,
    vec: &Vec<MetaTerm>,
    arg_map: &ArgNameMap,
) -> Result<Term, CompileError> {
    Ok(if !vec.is_empty() {
        Term::Cons(
            info.clone(),
            transform_to_core_internal(env, &vec[0], arg_map)?.into(),
            transform_list_construction_to_core_list_internal(
                info,
                env,
                &vec.clone().into_iter().skip(1).collect(),
                arg_map,
            )?
            .into(),
        )
    } else {
        Term::Nil(info)
    })
}

pub fn transform_to_core(env: &MetaEnv, mt: &MetaTerm) -> Result<Term, CompileError> {
    transform_to_core_internal(env, mt, &HashMap::new())
}
