(defrec
  is_even (n) (if (eq n 0) true (is_odd (- n 1)))
  is_odd (n) (if (eq n 0) false (is_even (- n 1))))

(assert (eq (is_odd 0) false))
(assert (eq (is_odd 1) true))
(assert (eq (is_odd 2) false))
(assert (eq (is_odd 3) true))

(assert (eq (is_even 0) true))
(assert (eq (is_even 1) false))
(assert (eq (is_even 2) true))
(assert (eq (is_even 3) false))
