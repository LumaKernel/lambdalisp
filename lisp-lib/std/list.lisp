(defrec len (xs)
  (if (eq xs nil)
    0
    (+ (len (cdr xs)) 1)))

(export len)

(assert (eq (len nil) 0))
(assert (eq (len (cons 1 nil)) 1))
(assert (eq (len (cons nil nil)) 1))
(assert (eq (len (list nil nil)) 2))
(assert (eq (len (list 1 (list 10 20 30) 3)) 3))
(assert (eq (len (list 1 2 3 4 5)) 5))



(defrec index (i xs)
  (if (eq i 0)
    (car xs)
    (index (- i 1) (cdr xs))))

index

(export index)

(assert (eq (index 0 (list 1 2 3 4 5)) 1))
(assert (eq (index 2 (list 1 2 3 4 5)) 3))
(assert (eq (index 3 (list 1 2 3 4 5)) 4))



(defrec append (xs v)
  (if (eq xs nil)
    (list v)
    (cons (car xs) (append (cdr xs) v))))

(defrec join (xs ys)
  (if (eq ys nil)
    xs
    (join (append xs (car ys)) (cdr ys))))

(export index)

(assert (eq (index 0 (list 1 2 3 4 5)) 1))
(assert (eq (index 2 (list 1 2 3 4 5)) 3))
(assert (eq (index 3 (list 1 2 3 4 5)) 4))
