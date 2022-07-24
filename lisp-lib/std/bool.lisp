(def nand (lambda (a b) (if a (if b false true) true)))
(def not (lambda (a) (if a false true)))
(def and (lambda (a b) (if a (if b true false) false)))
(def or (lambda (a b) (if a true (if b true false))))
(def nor (lambda (a b) (if a false (if b false true))))
(def xor (lambda (a b) (if a (if b false true) (if b true false))))
(def nxor (lambda (a b) (if a (if b true false) (if b false true))))

(export nand not and or nor xor nxor)
