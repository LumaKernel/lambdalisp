pub mod common;
pub mod corelang;
pub mod metalang;
use common::fileinfo::CompileError;
use metalang::parser::MetaParser;
use metalang::syntax::MetaStatement;

pub fn parse_string(str: String) -> Result<Vec<MetaStatement>, CompileError> {
    let mut p = MetaParser::new(str.chars().collect());
    p.parse_stmts()
}
