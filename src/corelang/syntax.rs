use crate::common::fileinfo::FileInfo;
use num_bigint::BigUint;

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

    // structure
    Cons(Option<FileInfo>, Box<Term>, Box<Term>),
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

    // bool op
    If(Option<FileInfo>),

    // structure op
    Car(Option<FileInfo>),
    Cdr(Option<FileInfo>),
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

            Term::Eq(info) => Term::Eq(f((*info).clone())),

            // structure
            Term::Cons(info, t1, t2) => {
                Term::Cons(f((*info).clone()), (*t1).clone(), (*t2).clone())
            }
            Term::Nil(info) => Term::Nil(f((*info).clone())),

            // arith
            Term::Number(info, n1) => Term::Number(f((*info).clone()), (*n1).clone()),

            // bool
            Term::Bool(info, b1) => Term::Bool(f((*info).clone()), *b1),

            // meta op
            Term::Eval(info) => Term::Eval(f((*info).clone())),

            // bool op
            Term::If(info) => Term::If(f((*info).clone())),

            // arith op
            Term::Add(info) => Term::Add(f((*info).clone())),
            Term::Sub(info) => Term::Sub(f((*info).clone())),

            // structure op
            Term::Car(info) => Term::Car(f((*info).clone())),
            Term::Cdr(info) => Term::Cdr(f((*info).clone())),
        }
    }

    pub fn map_subterm<F>(&self, f: F) -> Self
    where
        F: Fn(Term) -> Term,
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
pub fn shift_index(t: &Term, c: usize, d: usize) -> Term {
    match t {
        Term::Variable(info, v, a) => {
            Term::Variable((*info).clone(), if *v < c { *v } else { v + d }, *a)
        }
        Term::Lambda(info, arg_num, body) => Term::Lambda(
            (*info).clone(),
            *arg_num,
            shift_index(&**body, c + 1, d).into(),
        ),

        _ => t.map_subterm(|s| shift_index(&s, c, d)),
    }
}

pub fn equiv_term_vec(t1: &Vec<Term>, t2: &Vec<Term>) -> bool {
    use std::iter::zip;
    if t1.len() != t2.len() {
        return false;
    }
    !zip(t1, t2).any(|(e1, e2)| !equiv_term(e1, e2))
}

pub fn equiv_term(t1: &Term, t2: &Term) -> bool {
    match t1 {
        Term::Apply(_, t11, ts1) => {
            if let Term::Apply(_, t21, ts2) = t2 {
                return equiv_term(&**t11, &**t21) && equiv_term_vec(ts1, ts2);
            }
        }
        Term::Lambda(_, a1, t11) => {
            if let Term::Lambda(_, a2, t21) = t2 {
                return a1 == a2 && equiv_term(&**t11, &**t21);
            }
        }

        Term::Quote(_, t11) => {
            if let Term::Quote(_, t21) = t2 {
                return equiv_term(&**t11, &**t21);
            }
        }
        Term::Variable(_, v1, a1) => {
            if let Term::Variable(_, v2, a2) = t2 {
                return v1 == v2 && a1 == a2;
            }
        }

        Term::Eq(_) => {
            if let Term::Eq(_) = t2 {
                return true;
            }
        }

        // structure
        Term::Cons(_, t11, t12) => {
            if let Term::Cons(_, t21, t22) = t2 {
                return equiv_term(&**t11, &**t21) && equiv_term(&**t12, &**t22);
            }
        }
        Term::Nil(_) => {
            if let Term::Nil(_) = t2 {
                return true;
            }
        }

        // arith
        Term::Number(_, n1) => {
            if let Term::Number(_, n2) = t2 {
                return n1 == n2;
            }
        }

        // bool
        Term::Bool(_, b1) => {
            if let Term::Bool(_, b2) = t2 {
                return b1 == b2;
            }
        }

        // meta op
        Term::Eval(_) => {
            if let Term::Eval(_) = t2 {
                return true;
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

        // structure op
        Term::Car(_) => {
            if let Term::Car(_) = t2 {
                return true;
            }
        }
        Term::Cdr(_) => {
            if let Term::Cdr(_) = t2 {
                return true;
            }
        }
    }
    false
}

pub fn substitution(term: &Term, from: usize, to_vec: &Vec<Term>) -> Term {
    match term {
        Term::Variable(_, v1, w1) if *v1 == from => to_vec[*w1].clone(),
        Term::Lambda(info, arg_num, body) => Term::Lambda(
            info.clone(),
            *arg_num,
            substitution(
                &**body,
                from + 1,
                &to_vec.iter().map(|e| shift_index(e, 0, 1)).collect(),
            )
            .into(),
        ),

        _ => term.map_subterm(|s| substitution(&s, from, to_vec)),
    }
}
