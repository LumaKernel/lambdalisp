use super::super::syntax::Term;

pub struct SimplePrinter {}

impl SimplePrinter {
    pub fn new() -> Self {
        Self {}
    }

    fn print_vec(&self, vec: &[Term]) -> String {
        vec.iter()
            .map(|e| self.print(e))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn print(&self, term: &Term) -> String {
        match term {
            Term::Apply(_, t, ts) => format!("({} {})", self.print(t), self.print_vec(ts)),
            Term::Lambda(_, a, body) => format!("(lambda_{} {})", a, self.print(body)),

            Term::Quote(_, t) => format!("(quote {})", self.print(t)),
            Term::Variable(_, v, a) => format!("VAR<{},{}>", v, a),
            Term::Eq(_) => "eq".into(),

            Term::Cons(_, t1, t2) => format!("(const {} {})", self.print(t1), self.print(t2)),
            Term::Nil(_) => "nil".into(),

            Term::Number(_, n) => n.to_string(),

            Term::Bool(_, b) => b.to_string(),

            Term::Eval(_) => "eval".into(),

            Term::Add(_) => "add".into(),
            Term::Sub(_) => "sub".into(),

            Term::If(_) => "if".into(),

            Term::Car(_) => "car".into(),
            Term::Cdr(_) => "cdr".into(),
        }
    }
}
