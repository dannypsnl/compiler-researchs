#lang s-exp "syntax.rkt"

(λ (a) "")
(λ () #t)
(λ (a b) 1)

(let ([a 1]
      [b (λ (x) x)])
  (b a))
