use crate::textual_defs::{boolexp, exp, nodename};

/*
[OCaml Definition]
module Terminator : sig
  type node_call = {label: NodeName.t; ssa_args: Exp.t list}

  type t =
    | If of {bexp: BoolExp.t; then_: t; else_: t}
    | Ret of Exp.t
    | Jump of node_call list  (** non empty list *)
    | Throw of Exp.t
    | Unreachable
end
*/
pub struct NodeCall {
    label: nodename::T,
    ssa_args: Vec<exp::T>
}

pub enum T {
    If{bexp: boolexp::T, then: Box<T>, else_: Box<T>},
    Ret(exp::T),
    Jump(Vec<NodeCall>),
    Throw(exp::T),
    Unreachable
}