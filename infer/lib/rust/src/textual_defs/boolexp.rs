use crate::textual_defs::exp;

/*
[OCaml Definition]
module BoolExp : sig
  type t = Exp of Exp.t | Not of t | And of t * t | Or of t * t

  val pp : F.formatter -> t -> unit [@@warning "-unused-value-declaration"]
end
*/
#[derive(Debug)]
pub enum BoolExp {
    Exp(exp::Exp),
    Not(Box<BoolExp>),
    And(Box<BoolExp>,Box<BoolExp>),
    Or(Box<BoolExp>,Box<BoolExp>)
}