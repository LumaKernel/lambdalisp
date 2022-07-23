(def nand (lambda (a b) (if a (if b false true) true)))
(def not (lambda (a b) (if a false true)))
(def and (lambda (a b) (if a (if b true false) false)))
(def or (lambda (a b) (if a true (if b true false))))
(def nor (lambda (a b) (if a false (if b false true))))
(def xor (lambda (a b) (if a (if b false true) (if b true false))))

(def <= (lambda (a b) (not (< b a))))
(def >= (lambda (a b) (not (< a b))))
(def > (lambda (a b) (not (<= a b))))


(eq (cons 1 (+ 2 3)) (cons 1 5))
> true

(eq (lambda (a) eq) (lambda (b) eq))
> true

(eq (lambda (a) (+ 1 1)) (lambda (b) 2))
> false

(lambda (a b) (+ a (+ b 1)))

((lambda (a b) (+ a (+ b 1))) 3 4 5)  ; meta

(quote (lambda (a b) (+ a (+ b 1)) 3 4))
(lambda (x) (quote (lambda (a b) (+ a (+ b x)) x 4)))


((lambda (x y) (eq (quote (x)) (quote (y)))) eq eq)

(def factorial (lambda (n) (if (= n 0) 1 (factorial (- n 1)))))
(defrec factorial (n) (if (= n 0) 1 (factorial (- n 1))))
(def factorial
  (lambda (n)
    (
      (lambda (f n) (f f n))
      (lambda (self n) (if (= n 0) 1 (* n (self self (- n 1)))))
      n)))


; a * b = a + (a * (b - 1))
(defrec * (a b)
  (if (= a 0)
    0
    (+ a (* a (- b 1)))))

(defrec / (a b)
  (if (= b 0)
    nil
    (if (a < b)
      0
      (+ 1 (/ (- a b) b)))))

(defrec % (a b)
  (if (= b 0)
    nil
    (if (a < b)
      a
      (+ 1 (% (- a b) b)))))

(def index (lambda (xs i)
  (if (= i 0)
    (head xs)
    (index (tail xs) (- i 1)))))
(def len (lambda (xs)
  (if (= xs nil)
    0
    (+ 1 (len (tail xs))))))

(cons a (cons b nil))


メタ言語のトップレベル構文: def, defrec
メタ言語の構文: list

(list 1 2 3)

(let f be (lambda (x) (x x)) in
  (f f))
(def isodd
  (lambda (n)
    (
      (lambda (isodd iseven n) (isodd isodd iseven n))
      (lambda (self iseven n) (if (= n 0) false (iseven self iseven (- n 1))))
      (lambda (isodd self n) (if (= n 0) true (isodd isodd self (- n 1))))
      n)))
(defrec
  ideven (n) (if (= n 0) true (isodd (- n 1)))
  idodd (n) (if (= n 0) false (iseven (- n 1))))
