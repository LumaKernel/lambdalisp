mod base {}

pub mod core {

    // semantic
}

use churchlisp::common::fileinfo::CompileError;
use churchlisp::corelang::printer::simple::SimplePrinter;
use churchlisp::metalang::eval::eval_all;
use churchlisp::metalang::syntax::MetaEnv;
use churchlisp::parse_string;

fn parse_eval_print(str: String) -> Result<String, CompileError> {
    let env = MetaEnv::default();
    let (_, vs) = eval_all(&env, &parse_string(str)?)?;
    Ok(vs
        .into_iter()
        .map(|v| {
            let mut printer = SimplePrinter::new();
            printer.print(&v)
        })
        .collect::<Vec<String>>()
        .join("\n"))
}

fn print_result(str: &str) {
    match parse_eval_print(str.to_string()) {
        Ok(output) => {
            println!("{} = {}", str, output);
        }
        Err(err) => {
            println!("{}\n  ERROR: {}", str, err);
        }
    }
}

fn main() {
    println!("SUCCESS:");
    print_result("(+ 2 4)");
    print_result("(+ 2 (car (cons 8 3)))");
    print_result("((cdr (cons eq -)) 8888 3333 )");
    print_result("((car (cons eq -)) 8888 3333 )");
    println!();
    println!();
    println!();
    println!("ERROR:");
    print_result("(");
    print_result(")");
}
