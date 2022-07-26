(import "std")

(is_prime 83)

(defrec
  is_even (n) (if (eq n 0) true (is_odd (- n 1)))
  is_odd (n) (if (eq n 0) false (is_even (- n 1))))

is_even

(is_odd 3)
(is_even 3)
