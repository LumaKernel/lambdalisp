use super::super::syntax::{DefRecFun, MetaExport, MetaStatement, MetaTerm};

fn print_string(s: &String) -> String {
    format!(
        "\"{}\"",
        s.chars()
            // TODO another control chars
            .map(|c| match c {
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '"' => "\\\"".to_string(),
                _ => format!("{}", c),
            })
            .collect::<Vec<String>>()
            .join("")
    )
}

pub struct SimplePrinter {}

impl Default for SimplePrinter {
    fn default() -> Self {
        Self {}
    }
}

impl SimplePrinter {
    fn print_term_vec(&self, vec: &[MetaTerm]) -> String {
        vec.iter()
            .map(|e| self.print_term(e))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn print_arg_name_vec(&self, arg_name_vec: &[String]) -> String {
        arg_name_vec.join(" ")
    }

    pub fn print_stmt(&self, term: &MetaStatement) -> String {
        match term {
            MetaStatement::Def(_, name, def) => format!("(def {} {})", name, self.print_term(def)),
            MetaStatement::DefRec(_, fun_vec) => {
                format!("(defrec {})", self.print_defrec_fun_vec(fun_vec))
            }
            MetaStatement::Term(_, term) => self.print_term(term),
            MetaStatement::Assert(_, term) => format!("(assert {})", self.print_term(term)),
            MetaStatement::Import(_, import_vec) => {
                format!("(import {})", self.print_import_vec(import_vec))
            }
            MetaStatement::Export(_, export_vec) => {
                format!("(export {})", self.print_export_vec(export_vec))
            }
        }
    }

    pub fn print_defrec_fun_vec(&self, fun_vec: &Vec<DefRecFun>) -> String {
        fun_vec
            .iter()
            .map(|fun| self.print_defrec_fun(fun))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn print_defrec_fun(&self, fun: &DefRecFun) -> String {
        format!(
            "{} {} {}",
            fun.name,
            self.print_arg_name_vec(&fun.arg_name_vec),
            self.print_term(&fun.term)
        )
    }

    pub fn print_import_vec(&self, import_vec: &Vec<String>) -> String {
        import_vec
            .iter()
            .map(|import| self.print_import(import))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn print_import(&self, import: &String) -> String {
        print_string(import)
    }

    pub fn print_export_vec(&self, export_vec: &Vec<MetaExport>) -> String {
        export_vec
            .iter()
            .map(|export| self.print_export(export))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn print_export(&self, export: &MetaExport) -> String {
        match export {
            MetaExport::Var(v) => format!("{}", v),
            MetaExport::Path(p) => print_string(p),
        }
    }

    pub fn print_term(&self, term: &MetaTerm) -> String {
        match term {
            MetaTerm::Apply(_, t, ts) => {
                format!("({} {})", self.print_term(t), self.print_term_vec(ts))
            }
            MetaTerm::Lambda(_, arg_name_vec, body) => format!(
                "(lambda ({}) {})",
                self.print_arg_name_vec(arg_name_vec),
                self.print_term(body)
            ),

            MetaTerm::Quote(_, t) => format!("(quote {})", self.print_term(t)),
            MetaTerm::Variable(_, name) => name.clone(),
            MetaTerm::Eq(_) => "eq".into(),

            MetaTerm::If(_, t1, t2, t3) => format!(
                "(if {} {} {})",
                self.print_term(t1),
                self.print_term(t2),
                self.print_term(t3)
            ),

            MetaTerm::Cons(_, t1, t2) => {
                format!("(cons {} {})", self.print_term(t1), self.print_term(t2))
            }
            MetaTerm::Nil(_) => "nil".into(),

            MetaTerm::Number(_, n) => n.to_string(),

            MetaTerm::Bool(_, b) => b.to_string(),

            MetaTerm::Eval(_) => "eval".into(),

            MetaTerm::Add(_) => "+".into(),
            MetaTerm::Sub(_) => "-".into(),
            MetaTerm::Mul(_) => "*".into(),
            MetaTerm::Div(_) => "/".into(),
            MetaTerm::Rem(_) => "%".into(),

            MetaTerm::Car(_) => "car".into(),
            MetaTerm::Cdr(_) => "cdr".into(),

            MetaTerm::List(_, list) => format!("(list {})", self.print_term_vec(list)),
        }
    }
}
