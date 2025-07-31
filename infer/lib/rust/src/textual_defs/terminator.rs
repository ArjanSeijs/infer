use std::collections::HashMap;

use stable_mir::mir::TerminatorKind;

use crate::textual_defs::{
    PrintTextual, PrintTextualWithSeperator,
    boolexp::{self, BoolExp},
    exp::{self, Exp},
    ident,
    nodename::{self, NodeName},
};

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
            Terminator::If { bexp, then, else_ } => {
                format!(
                    "if {} then {} else {}",
                    bexp.pp(),
                    then.pp(),
                    else_.pp()
                )
            },
            Terminator::Ret(Exp::LVar(varname)) => format!("ret {}", varname.pp()),
            Terminator::Ret(exp) => format!("ret {}", exp.pp()),
            Terminator::Jump(node_calls) => format!("jmp {}", node_calls.pp_comma_list()),
            Terminator::Throw(exp) => format!("throw {}", exp.pp()),
            Terminator::Unreachable => "unreachable".into(),
        }
    }
}

impl PrintTextual for NodeCall {
    fn pp(&self) -> String {
        if self.ssa_args.is_empty() {
            self.label.pp()
        } else {
            format!("{}({})", self.label.pp(), self.ssa_args.pp_comma_list())
        }
    }
}

impl Terminator {
    pub fn jump(idx: &Option<usize>, label_map: &HashMap<usize, String>) -> Terminator {
        match idx {
            Some(idx) => {
                let label = label_map.get(&idx).unwrap();
                let nodecall = NodeCall {
                    label: NodeName::new(label.clone(), None),
                    ssa_args: vec![],
                };
                Terminator::Jump(vec![nodecall])
            }
            None => Terminator::Jump(vec![]),
        }
    }
}
