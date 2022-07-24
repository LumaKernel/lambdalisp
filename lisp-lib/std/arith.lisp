(def >= (lambda (a b) (eq (- b a) 0)))
(def <= (lambda (a b) (eq (- a b) 0)))

(def < (lambda (a b) (if (eq (- b a) 0) false true)))
(def > (lambda (a b) (if (eq (- a b) 0) false true)))

(export < <= >= >)

(assert (eq (< 1 2) true))
(assert (eq (< 2 1) false))
(assert (eq (< 0 0) false))

(assert (eq (<= 1 2) true))
(assert (eq (<= 1 1) true))
(assert (eq (<= 2 1) false))

(assert (eq (> 1 2) false))
(assert (eq (> 1 1) false))
(assert (eq (> 2 1) true))

(assert (eq (>= 1 2) false))
(assert (eq (>= 1 1) true))
(assert (eq (>= 2 1) true))



(defrec factorial (n) (if (eq n 0) 1 (* n (factorial (- n 1)))))

(export factorial)

(assert (eq (factorial 0) 1))
(assert (eq (factorial 1) 1))
(assert (eq (factorial 2) 2))
(assert (eq (factorial 3) 6))
(assert (eq (factorial 4) 24))
(assert (eq (factorial 5) 120))



(defrec _is_prime_sub (n k)
  (if (> (* k k) n)
    true
    (if (eq (% n k) 0)
      false
      (_is_prime_sub n (+ k 1)))))

(def is_prime (lambda (n) (if (< n 2) false (_is_prime_sub n 2))))

(export is_prime)

(assert (eq (is_prime 0) false))
(assert (eq (is_prime 1) false))
(assert (eq (is_prime 2) true))
(assert (eq (is_prime 3) true))
(assert (eq (is_prime 4) false))
(assert (eq (is_prime 5) true))
(assert (eq (is_prime 6) false))
(assert (eq (is_prime 7) true))
(assert (eq (is_prime 8) false))
(assert (eq (is_prime 9) false))
(assert (eq (is_prime 10) false))
(assert (eq (is_prime 11) true))
(assert (eq (is_prime 111) false))
(assert (eq (is_prime 1111) false))
(assert (eq (is_prime 3907) true))
