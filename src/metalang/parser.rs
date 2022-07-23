use crate::common::fileinfo::CompileError;
use crate::metalang::syntax::{MetaStatement, MetaTerm};
use num_bigint::BigUint;

pub struct MetaParser {
    p: usize,
    line: usize,
    col: usize,
    chars: Vec<char>,
}

fn is_identifier_head(c: char) -> bool {
    is_identifier(c) && !c.is_digit(10)
}
fn is_identifier(c: char) -> bool {
    !c.is_control() && !c.is_whitespace() && c != '(' && c != ')'
}

fn display_char(c: Option<char>) -> String {
    match c {
        Some(cc) => format!("'{}'", cc),
        None => "<EOF>".into(),
    }
}

impl MetaParser {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            p: 0,
            line: 0,
            col: 0,
            chars,
        }
    }

    fn get(&mut self) -> Option<char> {
        if self.chars.len() <= self.p {
            return None;
        }
        let b = self.chars[self.p];
        self.p += 1;
        Some(b)
    }
    fn peek(&mut self) -> Option<char> {
        if self.chars.len() <= self.p {
            return None;
        }
        Some(self.chars[self.p])
    }
    fn get_and_revert<V, F>(&mut self, f: F, ex: usize) -> V
    where
        F: FnOnce(&mut Self) -> V,
    {
        let p = self.p;
        let line = self.line;
        let col = self.col;
        for _ in 0..ex {
            self.get();
        }
        let v = f(self);
        self.p = p;
        self.line = line;
        self.col = col;
        v
    }

    fn get_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut id = Vec::<char>::new();
        while let Some(c) = self.peek() {
            if !f(c) {
                break;
            }
            id.push(self.get().unwrap());
        }
        id.iter().collect()
    }
    fn get_identifier(&mut self) -> String {
        if let Some(c) = self.peek() {
            if is_identifier_head(c) {
                return self.get_while(is_identifier);
            }
        }
        "".into()
    }
    fn get_decimal_number(&mut self) -> String {
        self.get_while(|c| c.is_ascii_digit())
    }
    fn peek_identifier(&mut self, ex: usize) -> String {
        self.get_and_revert(|p| p.get_identifier(), ex)
    }
    fn get_whitespaces(&mut self) -> String {
        self.get_while(|c| c.is_whitespace())
    }

    // parser combinators
    pub fn parse_stmts(&mut self) -> Result<Vec<MetaStatement>, CompileError> {
        let mut stmts = Vec::<MetaStatement>::new();
        while self.peek() != None {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }
    pub fn parse_stmt(&mut self) -> Result<MetaStatement, CompileError> {
        self.get_whitespaces();
        Ok(MetaStatement::Term(None, self.parse_term()?))
        // TODO:
        // if self.peek() == Some('(') {
        //     match self.get_identifier() {
        //         // "def" =>
        //         // "defrec" =>
        //     }
        // }
    }
    pub fn parse_term(&mut self) -> Result<MetaTerm, CompileError> {
        self.get_whitespaces();
        if self.peek() == Some('(') {
            self.get();
            self.get_whitespaces();
            let id = self.peek_identifier(0);
            let t = {
                match id.as_str() {
                    "lambda" => self.parse_term_lambda(),
                    "cons" => self.parse_term_cons(),
                    "list" => self.parse_term_list_direct(),
                    "quote" => self.parse_term_quote(),
                    _ => self.parse_term_apply(),
                }
            }?;
            self.get_whitespaces();
            {
                let c = self.get();
                if c != Some(')') {
                    return Err(CompileError {
                        info: None, // TODO
                        message: Some(format!(
                            "parenthesis for {} expected; got {}",
                            id,
                            display_char(c)
                        )),
                    });
                }
            }
            Ok(t)
        } else {
            self.parse_term_value()
        }
    }

    fn parse_term_lambda(&mut self) -> Result<MetaTerm, CompileError> {
        if self.get_identifier() != "lambda" {
            return Err(CompileError {
                info: None,
                message: Some("UNREACHABLE: lambda".into()),
            });
        }
        self.get_whitespaces();
        {
            let c = self.get();
            if c != Some('(') {
                return Err(CompileError {
                    info: None, // TODO
                    message: Some(format!(
                        "lambda start parenthesis for args expected; got {}",
                        display_char(c)
                    )),
                });
            }
        }
        self.get_whitespaces();
        let names = self.parse_names()?;
        self.get_whitespaces();
        {
            let c = self.get();
            if c != Some(')') {
                return Err(CompileError {
                    info: None, // TODO
                    message: Some(format!(
                        "lambda end parenthesis for args expected; got {}",
                        display_char(c)
                    )),
                });
            }
        }
        self.get_whitespaces();
        let body = self.parse_term()?;
        Ok(MetaTerm::Lambda(None, names, body.into()))
    }

    fn parse_term_apply(&mut self) -> Result<MetaTerm, CompileError> {
        let terms = self.parse_terms()?;
        if terms.is_empty() {
            return Err(CompileError {
                info: None,
                message: Some("Apply without operator not allowed".into()),
            });
        }
        Ok(MetaTerm::Apply(
            None,
            terms[0].clone().into(),
            terms.into_iter().skip(1).collect(),
        ))
    }

    fn parse_term_cons(&mut self) -> Result<MetaTerm, CompileError> {
        if self.get_identifier() != "cons" {
            return Err(CompileError {
                info: None, // TODO
                message: Some("cons expected".into()),
            });
        }
        let terms = self.parse_terms()?;
        if terms.len() != 2 {
            return Err(CompileError {
                info: None, // TODO
                message: Some("cons only accepts 2 terms".into()),
            });
        }
        self.get_whitespaces();
        Ok(MetaTerm::Cons(
            None,
            terms[0].clone().into(),
            terms[1].clone().into(),
        ))
    }

    fn parse_term_list_direct(&mut self) -> Result<MetaTerm, CompileError> {
        if self.get_identifier() != "list" {
            return Err(CompileError {
                info: None, // TODO
                message: Some("list expected".into()),
            });
        }
        let terms = self.parse_terms()?;
        self.get_whitespaces();
        Ok(MetaTerm::List(None, terms))
    }

    fn parse_term_quote(&mut self) -> Result<MetaTerm, CompileError> {
        if self.get_identifier() != "quote" {
            return Err(CompileError {
                info: None, // TODO
                message: Some("quote expected".into()),
            });
        }
        let terms = self.parse_terms()?;
        if terms.len() != 1 {
            return Err(CompileError {
                info: None, // TODO
                message: Some("quote only accepts 1 term".into()),
            });
        }
        self.get_whitespaces();
        Ok(MetaTerm::Quote(None, terms[0].clone().into()))
    }

    fn parse_term_value(&mut self) -> Result<MetaTerm, CompileError> {
        let id = self.get_identifier();
        Ok(if !id.is_empty() {
            match id.as_str() {
                "nil" => MetaTerm::Nil(None),
                "true" => MetaTerm::Bool(None, true),
                "false" => MetaTerm::Bool(None, false),
                "car" => MetaTerm::Car(None),
                "cdr" => MetaTerm::Cdr(None),
                "if" => MetaTerm::If(None),
                "eval" => MetaTerm::Eval(None),
                "eq" => MetaTerm::Eq(None),
                "+" => MetaTerm::Add(None),
                "-" => MetaTerm::Sub(None),
                _ => MetaTerm::Variable(None, id),
            }
        } else {
            let n = self.get_decimal_number();
            if !n.is_empty() {
                MetaTerm::Number(None, BigUint::parse_bytes(n.as_bytes(), 10).unwrap())
            } else {
                Err(CompileError {
                    info: None, // TODO
                    message: Some("value expected".into()),
                })?
            }
        })
    }

    fn parse_names(&mut self) -> Result<Vec<String>, CompileError> {
        let mut names = Vec::<String>::new();
        loop {
            let id = self.get_identifier();
            if id.is_empty() {
                break;
            }
            self.get_whitespaces();
            names.push(id);
        }
        Ok(names)
    }

    fn parse_terms(&mut self) -> Result<Vec<MetaTerm>, CompileError> {
        let mut terms = Vec::<MetaTerm>::new();
        loop {
            let peek = self.peek();
            if peek == None || peek == Some(')') {
                break;
            }
            let term = self.parse_term()?;
            self.get_whitespaces();
            terms.push(term);
        }
        Ok(terms)
    }
}
