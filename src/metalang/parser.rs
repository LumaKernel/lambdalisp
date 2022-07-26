use crate::common::fileinfo::{CompileError, FileInfo, Location, Range};
use crate::metalang::syntax::{DefRecFun, MetaExport, MetaStatement, MetaTerm};
use num_bigint::BigUint;

pub struct MetaParser {
    filepath: String,
    p: usize,
    line: usize,
    col: usize,
    chars: Vec<char>,
}

const RESERVED_NAMES: [&str; 15] = [
    "if", "+", "-", "eval", "quote", "lambda", "import", "exoprt", "assert", "print", "println",
    "for", "loop", "do", "while",
];

fn is_reserved_name(s: &str) -> bool {
    RESERVED_NAMES.into_iter().find(|e| s == *e) != None
}

fn is_identifier_head(c: char) -> bool {
    is_identifier(c) && !c.is_digit(10)
}
fn is_identifier(c: char) -> bool {
    !c.is_control() && !c.is_whitespace() && c != '(' && c != ')' && c != '"' && c != '\''
}

fn print_char(c: Option<char>) -> String {
    match c {
        Some(cc) => format!("'{}'", cc),
        None => "<EOF>".into(),
    }
}

impl MetaParser {
    pub fn new(filepath: String, chars: Vec<char>) -> Self {
        Self {
            filepath,
            p: 0,
            line: 0,
            col: 0,
            chars,
        }
    }

    fn loc(&self) -> Location {
        Location {
            line: self.line,
            col: self.col,
        }
    }
    fn locinfo(&self) -> Option<FileInfo> {
        Some(FileInfo {
            range: Range {
                from: self.loc(),
                to: self.loc(),
            },
            filepath: self.filepath.clone(),
        })
    }
    fn rangeinfo(&self, n: usize) -> Option<FileInfo> {
        Some(FileInfo {
            range: Range {
                from: Location {
                    line: self.loc().line,
                    col: self.loc().col - n,
                },
                to: self.loc(),
            },
            filepath: self.filepath.clone(),
        })
    }
    fn get(&mut self) -> Option<char> {
        if self.chars.len() <= self.p {
            return None;
        }
        let b = self.chars[self.p];
        self.p += 1;
        if b == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(b)
    }
    fn peek(&mut self) -> Option<char> {
        if self.chars.len() <= self.p {
            return None;
        }
        Some(self.chars[self.p])
    }
    fn store(&self) -> (usize, usize, usize) {
        (self.p, self.line, self.col)
    }
    fn restore(&mut self, store: (usize, usize, usize)) {
        self.p = store.0;
        self.line = store.1;
        self.col = store.2;
    }

    fn skip(&mut self) -> String {
        self.get_while(|c| c.is_whitespace())
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
    fn get_end_parenthesis(&mut self, message: &str) -> Result<(), CompileError> {
        let c = self.get();
        if c != Some(')') {
            return Err(CompileError {
                info: self.locinfo(),
                message: Some(format!("{}: found {}", message, print_char(c))),
            });
        }
        Ok(())
    }

    // parser combinators
    fn parse_decimal_number(&mut self) -> Option<BigUint> {
        let n = self.get_while(|c| c.is_ascii_digit());
        if n.is_empty() {
            None
        } else {
            Some(BigUint::parse_bytes(n.as_bytes(), 10).unwrap())
        }
    }
    fn parse_name_define(&mut self) -> Result<Option<String>, CompileError> {
        let id = self.parse_identifier();
        if let Some(id) = &id {
            if is_reserved_name(id) {
                return Err(CompileError {
                    info: self.locinfo(),
                    message: Some(format!(
                        "expected string starting double quotation: found {}",
                        print_char(self.peek())
                    )),
                });
            }
        }
        Ok(id)
    }
    fn parse_identifier(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            if is_identifier_head(c) {
                return Some(self.get_while(is_identifier));
            }
        }
        None
    }

    fn parse_string(&mut self) -> Result<String, CompileError> {
        if self.get() != Some('"') {
            return Err(CompileError {
                info: self.locinfo(),
                message: Some(format!(
                    "expected string starting double quotation: found {}",
                    print_char(self.peek())
                )),
            });
        }
        let mut s = String::new();
        while let Some(c) = self.get() {
            match c {
                '\\' => match self.get() {
                    Some(escaped) => match escaped {
                        '\\' => s.push('\\'),
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        _ => {
                            return Err(CompileError {
                                info: self.locinfo(),
                                message: Some(format!(
                                    "unknown string escape sequence \"\\{}\"",
                                    escaped,
                                )),
                            })
                        }
                    },
                    None => {
                        return Err(CompileError {
                            info: self.locinfo(),
                            message: Some(format!(
                                "expected string ending double quotation: found {}",
                                print_char(None)
                            )),
                        })
                    }
                },
                '"' => {
                    break;
                }
                _ => {
                    s.push(c);
                }
            }
        }
        Ok(s)
    }

    pub fn parse_stmt_vec(&mut self) -> Result<Vec<MetaStatement>, CompileError> {
        let mut stmt_vec = Vec::<MetaStatement>::new();
        while self.peek() != None {
            stmt_vec.push(self.parse_stmt()?);
            self.skip();
        }
        Ok(stmt_vec)
    }
    pub fn parse_stmt(&mut self) -> Result<MetaStatement, CompileError> {
        self.skip();
        if self.peek() == Some('(') {
            let store = self.store();
            'special_check: loop {
                self.get();
                self.skip();
                let id = match self.parse_identifier() {
                    Some(id) => id,
                    None => {
                        break 'special_check;
                    }
                };
                self.skip();
                match id.as_str() {
                    "def" => return self.parse_stmt_def(),
                    "defrec" => return self.parse_stmt_defrec(),
                    "import" => return self.parse_stmt_import(),
                    "export" => return self.parse_stmt_export(),
                    "assert" => return self.parse_stmt_assert(),
                    _ => break 'special_check,
                };
            }
            self.restore(store);
        }
        Ok(MetaStatement::Term(
            self.locinfo(), /* TODO: locinfo */
            self.parse_term()?,
        ))
    }

    fn parse_stmt_def(&mut self) -> Result<MetaStatement, CompileError> {
        self.skip();
        let id = match self.parse_name_define()? {
            Some(id) => id,
            None => {
                return Err(CompileError {
                    info: self.locinfo(),
                    message: Some("def should follow variable name".into()),
                });
            }
        };
        self.skip();
        let term = self.parse_term()?;
        self.skip();
        self.get_end_parenthesis("expect def statement end parenthesis")?;
        Ok(MetaStatement::Def(
            self.locinfo(), /* TODO: locinfo */
            id,
            term,
        ))
    }

    fn parse_stmt_defrec(&mut self) -> Result<MetaStatement, CompileError> {
        let fun_vec = self.parse_vec(|p| p.parse_defrec_fun().map(Some), "defrec definition")?;
        Ok(MetaStatement::DefRec(
            self.locinfo(), /* TODO: locinfo */
            fun_vec,
        ))
    }

    fn parse_defrec_fun(&mut self) -> Result<DefRecFun, CompileError> {
        let name = match self.parse_name_define()? {
            Some(name) => name,
            None => {
                return Err(CompileError {
                    info: self.locinfo(),
                    message: Some("expect defrec definition variable name".into()),
                });
            }
        };
        self.skip();
        let arg_name_vec = self.parse_arg_name_vec("defrec definition args")?;
        self.skip();
        let term = self.parse_term()?;
        Ok(DefRecFun {
            name,
            arg_name_vec,
            term,
        })
    }

    fn parse_stmt_import(&mut self) -> Result<MetaStatement, CompileError> {
        let string_vec = self.parse_string_vec()?;
        Ok(MetaStatement::Import(self.locinfo(), string_vec))
    }

    fn parse_string_vec(&mut self) -> Result<Vec<String>, CompileError> {
        self.parse_vec(|p| p.parse_string().map(Some), "string")
    }

    fn parse_stmt_export(&mut self) -> Result<MetaStatement, CompileError> {
        Ok(MetaStatement::Export(
            self.locinfo(),
            self.parse_vec(|p| p.parse_export().map(Some), "string or identifier")?,
        ))
    }

    fn parse_export(&mut self) -> Result<MetaExport, CompileError> {
        Ok(if self.peek() == Some('"') {
            MetaExport::Path(self.parse_string()?)
        } else {
            MetaExport::Var(self.parse_identifier().ok_or(CompileError {
                info: self.locinfo(),
                message: Some(
                    "export statement expects string or identifier for variable exported".into(),
                ),
            })?)
        })
    }

    fn parse_stmt_assert(&mut self) -> Result<MetaStatement, CompileError> {
        let term = self.parse_term()?;
        self.skip();
        self.get_end_parenthesis("expect assert statement end parenthesis")?;
        Ok(MetaStatement::Assert(self.locinfo(), term))
    }

    pub fn parse_term(&mut self) -> Result<MetaTerm, CompileError> {
        self.skip();
        if self.peek() == Some('(') {
            self.get();
            self.skip();
            let store = self.store();
            'special_check: loop {
                match {
                    let id = self.parse_identifier();
                    self.skip();
                    id
                } {
                    Some(id) => match id.as_str() {
                        "lambda" => return self.parse_term_lambda(),
                        "cons" => return self.parse_term_cons(),
                        "list" => return self.parse_term_list_direct(),
                        "quote" => return self.parse_term_quote(),
                        "if" => return self.parse_term_if(),
                        _ => break 'special_check,
                    },
                    None => break 'special_check,
                }
            }
            self.restore(store);
            self.parse_term_apply()
        } else {
            self.parse_term_value()
        }
    }

    fn parse_term_lambda(&mut self) -> Result<MetaTerm, CompileError> {
        let arg_name_vec = self.parse_arg_name_vec("lambda args")?;
        self.skip();
        let body = self.parse_term()?;
        self.skip();
        self.get_end_parenthesis("expect lambda end parenthesis for args")?;
        Ok(MetaTerm::Lambda(
            self.locinfo(), /* TODO: locinfo */
            arg_name_vec,
            body.into(),
        ))
    }

    fn parse_term_apply(&mut self) -> Result<MetaTerm, CompileError> {
        let terms = self.parse_term_vec()?;
        if terms.is_empty() {
            return Err(CompileError {
                info: self.locinfo(),
                message: Some("Apply without operator not allowed".into()),
            });
        }
        Ok(MetaTerm::Apply(
            self.locinfo(), /* TODO: locinfo */
            terms[0].clone().into(),
            terms.into_iter().skip(1).collect(),
        ))
    }

    fn parse_term_cons(&mut self) -> Result<MetaTerm, CompileError> {
        let terms = self.parse_term_vec()?;
        if terms.len() != 2 {
            return Err(CompileError {
                info: self.locinfo(),
                message: Some("cons only accepts 2 terms".into()),
            });
        }
        self.skip();
        Ok(MetaTerm::Cons(
            self.locinfo(), /* TODO: locinfo */
            terms[0].clone().into(),
            terms[1].clone().into(),
        ))
    }

    fn parse_term_list_direct(&mut self) -> Result<MetaTerm, CompileError> {
        let terms = self.parse_term_vec()?;
        self.skip();
        Ok(MetaTerm::List(
            self.locinfo(), /* TODO: locinfo */
            terms,
        ))
    }

    fn parse_term_quote(&mut self) -> Result<MetaTerm, CompileError> {
        let terms = self.parse_term_vec()?;
        if terms.len() != 1 {
            return Err(CompileError {
                info: self.locinfo(),
                message: Some("quote only accepts 1 term".into()),
            });
        }
        self.skip();
        Ok(MetaTerm::Quote(
            self.locinfo(), /* TODO: locinfo */
            terms[0].clone().into(),
        ))
    }

    fn parse_term_if(&mut self) -> Result<MetaTerm, CompileError> {
        let terms = self.parse_term_vec()?;
        if terms.len() != 3 {
            return Err(CompileError {
                info: self.locinfo(),
                message: Some("if only accepts 3 terms".into()),
            });
        }
        self.skip();
        Ok(MetaTerm::If(
            self.locinfo(), /* TODO: locinfo */
            terms[0].clone().into(),
            terms[1].clone().into(),
            terms[2].clone().into(),
        ))
    }

    fn parse_term_value(&mut self) -> Result<MetaTerm, CompileError> {
        Ok(match self.parse_identifier() {
            Some(id) => match id.as_str() {
                "nil" => MetaTerm::Nil(self.rangeinfo(id.len())),
                "true" => MetaTerm::Bool(self.rangeinfo(id.len()), true),
                "false" => MetaTerm::Bool(self.rangeinfo(id.len()), false),
                "car" => MetaTerm::Car(self.rangeinfo(id.len())),
                "cdr" => MetaTerm::Cdr(self.rangeinfo(id.len())),
                "eval" => MetaTerm::Eval(self.rangeinfo(id.len())),
                "eq" => MetaTerm::Eq(self.rangeinfo(id.len())),
                "+" => MetaTerm::Add(self.rangeinfo(id.len())),
                "-" => MetaTerm::Sub(self.rangeinfo(id.len())),
                "*" => MetaTerm::Mul(self.rangeinfo(id.len())),
                "/" => MetaTerm::Div(self.rangeinfo(id.len())),
                "%" => MetaTerm::Rem(self.rangeinfo(id.len())),
                _ => MetaTerm::Variable(self.rangeinfo(id.len()), id),
            },
            None => match self.parse_decimal_number() {
                Some(n) => MetaTerm::Number(self.locinfo() /* TODO: locinfo */, n),
                None => Err(CompileError {
                    info: self.locinfo(),
                    message: Some(format!("value expected: found {}", print_char(self.peek()))),
                })?,
            },
        })
    }

    fn parse_arg_name_vec(&mut self, message: &str) -> Result<Vec<String>, CompileError> {
        {
            let c = self.get();
            if c != Some('(') {
                return Err(CompileError {
                    info: self.locinfo(),
                    message: Some(format!(
                        "{}: expect start parenthesis for args: got {}",
                        message,
                        print_char(c)
                    )),
                });
            }
        }
        self.skip();
        self.parse_vec(|p| p.parse_name_define(), "identifier")
    }

    fn parse_term_vec(&mut self) -> Result<Vec<MetaTerm>, CompileError> {
        self.parse_vec(|p| p.parse_term().map(Some), "term")
    }

    fn parse_vec<T, F: Fn(&mut Self) -> Result<Option<T>, CompileError>>(
        &mut self,
        f: F,
        expect: &str,
    ) -> Result<Vec<T>, CompileError> {
        let mut vec = Vec::<T>::new();
        loop {
            let c = self.peek();
            if c == Some(')') {
                self.get();
                break;
            }
            let got = f(self)?.ok_or(CompileError {
                info: self.locinfo(),
                message: Some(format!("{} expected: found \"{}\"", expect, print_char(c))),
            })?;
            self.skip();
            vec.push(got);
        }
        Ok(vec)
    }
}
