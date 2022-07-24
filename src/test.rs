use crate::common::fileinfo::CompileError;
use crate::corelang::printer::simple::SimplePrinter;
use crate::metalang::eval::MetaEvaluator;
use crate::metalang::parser::MetaParser;
use crate::metalang::syntax::MetaStatement;

fn parse_string(str: String) -> Result<Vec<MetaStatement>, CompileError> {
    let mut p = MetaParser::new("<test>".into(), str.chars().collect());
    p.parse_stmt_vec()
}

fn parse_eval_print(str: String) -> Result<String, CompileError> {
    let mut evaluator = MetaEvaluator::default();
    let vs = evaluator.eval_vec(&parse_string(str)?)?;
    Ok(vs
        .into_iter()
        .map(|v| {
            let printer = SimplePrinter::default();
            printer.print(&v)
        })
        .collect::<Vec<String>>()
        .join("\n"))
}

fn test_success(source: &str, want: &str) {
    let got = match parse_eval_print(source.to_string()) {
        Ok(out) => out,
        Err(err) => format!("ERROR: {}", err),
    };
    assert_eq!(want, got, "\nsource:{}", source);
}

#[test]
fn test() {
    test_success("(- 2 4)", "0");
    test_success("(- 4 2)", "2");
    test_success("(+ 2 4)", "6");
    test_success("(* 2 4)", "8");
    test_success("(/ 2 4)", "0");
    test_success("(/ 111 23)", "4");
    test_success("(% 111 23)", "19");
    test_success(" ( +  1  0 )  ", "1");
    test_success("(+ 2 (car (cons 8 3)))", "10");
    test_success("((cdr (cons eq -)) 8888 3333 )", "5555");
    test_success("((car (cons eq -)) 8888 3333 )", "false");
    test_success("(lambda (a) a)", "(lambda<1-ary> ARG<0-up 0-th>)");
    test_success("((lambda (n) (+ 1 n)) 4)", "5");
    test_success("((lambda (n) (quote (+ 1 n))) 4)", "(quote (+ 1 4))");
    test_success("(eval ((lambda (n) (quote (+ 1 n))) 4))", "5");
    test_success("((lambda (n) (+ ((lambda (n) n) 10) n)) 4)", "14");
    test_success("(list)", "nil");
    test_success(
        "(list 1 2 3 4 5)",
        "(cons 1 (cons 2 (cons 3 (cons 4 (cons 5 nil)))))",
    );
    test_success(
        "(quote (list 1 2 3 4 5))",
        "(quote (cons 1 (cons 2 (cons 3 (cons 4 (cons 5 nil))))))",
    );
    test_success(
        "(cdr (cdr (list 1 2 3 4 5)))",
        "(cons 3 (cons 4 (cons 5 nil)))",
    );
    test_success("(car (cdr (cdr (list 1 2 3 4 5))))", "3");
    test_success("((lambda (a b) (+ a (+ b 1))) 10 100)", "111");
    test_success(
        "(eq (lambda (n) (lambda (n) n)) (lambda (m) (lambda (a) a)))",
        "true",
    );
    test_success(
        "(eq (lambda (n) (lambda (n) n)) (lambda (a) (lambda (m) a)))",
        "false",
    );
    test_success("(def a 1)a", "1");
    test_success("(def a 1) (def a 3) a", "3");
    test_success("(def a 1) (def a a) a", "1");
    test_success("(def abc nil) abc", "nil");

    test_success("(import \"std/arith\") (< 1 10)", "true");

    test_success("(import \"std\") (not false)", "true");
    test_success("(import \"std\") (not true)", "false");

    test_success("(import \"std\") (and false false)", "false");
    test_success("(import \"std\") (and true false)", "false");
    test_success("(import \"std\") (and false true)", "false");
    test_success("(import \"std\") (and true true)", "true");

    test_success("(import \"std\") (nand false false)", "true");
    test_success("(import \"std\") (nand true false)", "true");
    test_success("(import \"std\") (nand false true)", "true");
    test_success("(import \"std\") (nand true true)", "false");

    test_success("(import \"std\") (or false false)", "false");
    test_success("(import \"std\") (or true false)", "true");
    test_success("(import \"std\") (or false true)", "true");
    test_success("(import \"std\") (or true true)", "true");

    test_success("(import \"std\") (nor false false)", "true");
    test_success("(import \"std\") (nor true false)", "false");
    test_success("(import \"std\") (nor false true)", "false");
    test_success("(import \"std\") (nor true true)", "false");

    test_success("(import \"std\") (xor false false)", "false");
    test_success("(import \"std\") (xor true false)", "true");
    test_success("(import \"std\") (xor false true)", "true");
    test_success("(import \"std\") (xor true true)", "false");

    test_success("(import \"std\") (nxor false false)", "true");
    test_success("(import \"std\") (nxor true false)", "false");
    test_success("(import \"std\") (nxor false true)", "false");
    test_success("(import \"std\") (nxor true true)", "true");
}
