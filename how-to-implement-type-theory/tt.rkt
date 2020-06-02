#lang typed/racket

(define-type ty exp)
(struct exp () #:transparent)
(struct exp:var exp ([idx : Integer]) #:transparent)
(struct exp:typ exp () #:transparent)
(struct exp:prod exp
  ([x : (Pair String ty)]
   [m : ty])
  #:transparent)
(struct exp:lambda exp
  ([x : (Pair String (Option ty))]
   [m : exp])
  #:transparent)
(struct exp:app exp
  ([e1 : exp]
   [e2 : exp])
  #:transparent)
(struct exp:ascribe exp
  ([e : exp]
   [t : ty])
  #:transparent)