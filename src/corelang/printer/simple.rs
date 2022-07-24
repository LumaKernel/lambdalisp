use super::super::syntax::Term;

pub struct SimplePrinter {}

impl Default for SimplePrinter {
    fn default() -> Self {
        Self {}
    }
}

impl SimplePrinter {
    fn print_vec(&self, vec: &[Term]) -> String {
        vec.iter()
            .map(|e| self.print(e))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn print(&self, term: &Term) -> String {
        match term {
            Term::Apply(_, t, ts) => format!("({} {})", self.print(t), self.print_vec(ts)),
            Term::Lambda(_, a, body) => format!("(lambda<{}-ary> {})", a, self.print(body)),

            Term::Quote(_, t) => format!("(quote {})", self.print(t)),
            Term::Variable(_, v, a) => format!("ARG<{}-up {}-th>", v, a),
            Term::Eq(_) => "eq".into(),

            Term::If(_, t1, t2, t3) => format!(
                "(if {} {} {})",
                self.print(t1),
                self.print(t2),
                self.print(t3)
            ),

            Term::Cons(_, t1, t2) => format!("(cons {} {})", self.print(t1), self.print(t2)),
            Term::Nil(_) => "nil".into(),

            Term::Number(_, n) => n.to_string(),

            Term::Bool(_, b) => b.to_string(),

            Term::Eval(_) => "eval".into(),

            Term::Add(_) => "+".into(),
            Term::Sub(_) => "-".into(),
            Term::Mul(_) => "*".into(),
            Term::Div(_) => "/".into(),
            Term::Rem(_) => "%".into(),

            Term::Car(_) => "car".into(),
            Term::Cdr(_) => "cdr".into(),
        }
    }
}
