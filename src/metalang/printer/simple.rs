use super::super::syntax::MetaTerm;

pub struct SimplePrinter {}

impl SimplePrinter {
    fn print_vec(vec: &[MetaTerm]) -> String {
        vec.iter()
            .map(Self::print)
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn print_arg_names(arg_names: &[String]) -> String {
        arg_names.join(" ")
    }

    pub fn print(term: &MetaTerm) -> String {
        match term {
            MetaTerm::Apply(_, t, ts) => format!("({} {})", Self::print(t), Self::print_vec(ts)),
            MetaTerm::Lambda(_, arg_names, body) => format!(
                "(lambda ({}) {})",
                Self::print_arg_names(arg_names),
                Self::print(body)
            ),

            MetaTerm::Quote(_, t) => format!("(quote {})", Self::print(t)),
            MetaTerm::Variable(_, name) => name.clone(),
            MetaTerm::Eq(_) => "eq".into(),

            MetaTerm::Cons(_, t1, t2) => {
                format!("(cons {} {})", Self::print(t1), Self::print(t2))
            }
            MetaTerm::Nil(_) => "nil".into(),

            MetaTerm::Number(_, n) => n.to_string(),

            MetaTerm::Bool(_, b) => b.to_string(),

            MetaTerm::Eval(_) => "eval".into(),

            MetaTerm::Add(_) => "add".into(),
            MetaTerm::Sub(_) => "sub".into(),

            MetaTerm::If(_) => "if".into(),

            MetaTerm::Car(_) => "head".into(),
            MetaTerm::Cdr(_) => "tail".into(),

            MetaTerm::List(_, list) => format!("(list {})", Self::print_vec(list)),
        }
    }
}
