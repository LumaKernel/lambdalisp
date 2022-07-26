# LambdaLISP

## Church LambdaLISP

PLANNED

- unary de Bruijn indexed lambda
  - No way to construct directly.
- apply
  - e.g. `(f arg1 arg2)`

## Core LambdaLISP

- Superset of Church LambdaLISP
- n-ary de Bruijn indexed lambda
  - No way to construct directly.
- apply
  - e.g. `(f arg1 arg2)`
- if (short circuit)
  - e.g. `(if cond then_clause else_clause)`
- values
  - number
    - e.g. `0`, `1`, ...
  - bool
    - e.g. `true`, `false`
  - cons
    - e.g. `(cons 0 (cons 1 2))`, `(cons (cons 1 2) (cons 3 4))`
- operators
  - arithmetic
    - `+` (add) 
    - `-` (positive integer subtract)
    - `*` (multiply)
    - `/` (integer divide)
    - `%` (remainder)
  - car / cdr

## Meta LambdaLISP

- All Core LambdaLISP features
- Extended term level syntax sugar notations
  - n-ary named args lambda
    - e.g. `(lambda (a b) (+ a b))`
  - list
    - e.g. `(list 1 2 3 4 5)` (equivalent to `(cons 1 (cons 2 (cons 3 (cons 4 (cons 5 nil)))))`)
  - let (TODO)
- Environment control statements
  - def
  - defrec
  - import
    - e.g. `(import "./path/to/lib.lisp")`, `(import "std/arith" "church/integer")`
  - reexport
  - export
    - e.g. `(export factorial is_prime)`
    - e.g. `(export "./path/to/lib.lisp")`, `(export "std/arith" "church/integer")`
- Assert statement
  - e.g. `(assert (eq v1 v2))`

## Built-in libraries

- `std`
  - `std/arith`
  - `std/bool`
  - `std/list`
- `pfds` (PLANNED)
  - `pfds/heap`
  - ...
- `church` (PLANNED)
  - `church/integer`
  - `church/bool`
  - `church/pair`


## TODO

- [ ] Refinement of quote
- [ ] CI (test, publish to registry)
- [ ] Implement Jupyter Kernel
- [ ] Web interaction page with wasm
- [ ] Walk-through for proving Gödel's incompleteness theorems with Chaitin's The Unknowable
- [ ] Prove Turing completeness of lambda calculus (prove that of LambdaLisp, construct conversion from LambdaLisp to lambda calculus)
