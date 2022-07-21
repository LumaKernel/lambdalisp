(define nand (lambda (a b) (if a (if b false true) true)))
(define not (lambda (a b) (if a false true)))
(define and (lambda (a b) (if a (if b true false) false)))
(define or (lambda (a b) (if a true (if b true false))))
(define nor (lambda (a b) (if a false (if b false true))))
(define xor (lambda (a b) (if a (if b false true) (if b true false))))

(define <= (lambda (a b) (not (< b a))))
(define >= (lambda (a b) (not (< a b))))
(define > (lambda (a b) (not (<= a b))))


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

(define factorial (n) (if (= n 0) 1 (factorial (- n 1))))
(lambda (n)
	(lambda (self n) (if (= n 0) 1 (* n (self self (- n 1)))))
	(lambda (self n) (if (= n 0) 1 (* n (self self (- n 1)))))
	n)

