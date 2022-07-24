use crate::corelang::printer::simple::SimplePrinter as CorePrinter;
use crate::metalang::eval::MetaEvaluator;
use crate::metalang::parser::MetaParser;
use crate::metalang::printer::simple::SimplePrinter as MetaPrinter;
use crate::resolver::fs::FsResolver;

use std::fs::File;
use std::io::prelude::*;

pub fn run(filepath: String, verbose: bool, do_assert: bool) {
    let mut content = String::new();
    File::open(&filepath)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let mut p = MetaParser::new(filepath.into(), content.chars().collect());

    let stmt_vec = &p.parse_stmt_vec().unwrap();
    let mut evaluator = MetaEvaluator::default();
    evaluator.set_resolver(Box::new(FsResolver::default()));
    evaluator.do_assert = do_assert;
    for (i, stmt) in stmt_vec.iter().enumerate() {
        let cp = CorePrinter::default();
        if verbose {
            let mp = MetaPrinter::default();
            println!(" In[{}] = {}", i, mp.print_stmt(stmt));
        }
        match evaluator.eval(stmt) {
            Ok(v) => match v {
                Some(term) => {
                    println!("Out[{}] = {}", i, cp.print(&term));
                }
                None => {}
            },
            Err(e) => {
                println!("EVAL ERROR: {}", e);
                std::process::exit(1);
            }
        }
    }
}
