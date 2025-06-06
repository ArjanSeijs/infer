use crate::textual_defs::exp;

/*
[OCaml Definition]
module BoolExp : sig
  type t = Exp of Exp.t | Not of t | And of t * t | Or of t * t

  val pp : F.formatter -> t -> unit [@@warning "-unused-value-declaration"]
end
*/
pub enum T {
    Exp(exp::T),
    Not(Box<T>),
    And(Box<T>,Box<T>),
    Or(Box<T>,Box<T>)
}