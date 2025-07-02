use stable_mir::mir::{TerminatorKind};

use crate::textual_defs::{boolexp, exp, ident, nodename, PrintTextual};

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
    pub ssa_args: Vec<exp::Exp>,
}

#[derive(Debug)]
pub enum Terminator {
    If {
        bexp: boolexp::BoolExp,
        then: Box<Terminator>,
        else_: Box<Terminator>,
    },
    Ret(exp::Exp),
    Jump(Vec<NodeCall>),
    Throw(exp::Exp),
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

pub fn terminator_to_textual(terminator: &stable_mir::mir::Terminator) -> Terminator {
    match &terminator.kind {
        TerminatorKind::Goto { target } => todo!("Goto"),
        TerminatorKind::SwitchInt { discr, targets } => todo!("SwitchInt"),
        TerminatorKind::Resume => todo!("Resume"),
        TerminatorKind::Abort => todo!("Abort"),
        TerminatorKind::Return => Terminator::Ret(exp::Exp::Var(ident::T { val: 0 })), // Rust puts return value in 0
        TerminatorKind::Unreachable => Terminator::Unreachable,
        TerminatorKind::Drop {
            place,
            target,
            unwind,
        } => todo!("Drop"),
        TerminatorKind::Call {
            func,
            args,
            destination,
            target,
            unwind,
        } => todo!("Call"),
        TerminatorKind::Assert {
            cond,
            expected,
            msg,
            target,
            unwind,
        } => todo!("Assert"),
        TerminatorKind::InlineAsm {
            template,
            operands,
            options,
            line_spans,
            destination,
            unwind,
        } => todo!("InlineAsm"),
    }
}
