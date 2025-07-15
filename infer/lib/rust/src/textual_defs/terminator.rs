use stable_mir::mir::{TerminatorKind};

use crate::textual_defs::{boolexp::{self, BoolExp}, exp::{self, Exp}, ident, nodename, PrintTextual};

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

#[derive(Debug)]
pub struct NodeCall {
    pub label: nodename::NodeName,
    pub ssa_args: Vec<Exp>,
}

#[derive(Debug)]
pub enum Terminator {
    If {
        bexp: BoolExp,
        then: Box<Terminator>,
        else_: Box<Terminator>,
    },
    Ret(Exp),
    Jump(Vec<NodeCall>),
    Throw(Exp),
    Unreachable,
}

impl PrintTextual for Terminator {
    fn pp(&self) -> String {
        match self {
            Terminator::If { bexp, then, else_ } => todo!(),
            Terminator::Ret(exp) => format!("ret {}", exp.pp()),
            Terminator::Jump(node_calls) => todo!(),
            Terminator::Throw(exp) => todo!(),
            Terminator::Unreachable => todo!(),
        }
    }
}
