use stable_mir::{
    mir::{
        BinOp, ConstOperand, Operand, Place,
        Rvalue::{self, BinaryOp, Use},
        Statement, StatementKind,
    },
    ty::{Allocation, ConstantKind::Allocated},
};

use crate::textual_defs::{
    constant, exp::{self, Exp}, ident::{self, Ident}, location::{self, Location}, name, procname, qualifiedprocname, typ::{self, Typ}, varname::VarName, PrintTextual
};

/*
[OCaml Definition]
module Instr : sig
  type t =
    | Load of {id: Ident.t; exp: Exp.t; typ: Typ.t option; loc: Location.t}
        (** id <- *exp with *exp:typ *)
    | Store of {exp1: Exp.t; typ: Typ.t option; exp2: Exp.t; loc: Location.t}
        (** *exp1 <- exp2 with exp2:typ *)
    | Prune of {exp: Exp.t; loc: Location.t}  (** assume exp *)
    | Let of {id: Ident.t option; exp: Exp.t; loc: Location.t}  (** id = exp or _ = exp *)
  (* Remark that because Sil operations (add, mult...) are calls, we let the Textual programmer put
     expression in local variables, while SIL forbid that. The to_sil transformation will have to
     inline these definitions. *)

  val loc : t -> Location.t

  val pp : ?show_location:bool -> F.formatter -> t -> unit
end
*/

#[derive(Debug)]
pub enum Instr {
    Load {
        id: Ident,
        exp: Exp,
        typ: Option<Typ>,
        loc: Location,
    },
    Store {
        exp1: Exp,
        typ: Option<Typ>,
        exp2: Exp,
        loc: Location,
    },
    Prune {
        exp: Exp,
        loc: Location,
    },
    Let {
        id: Option<Ident>,
        exp: Exp,
        loc: Location,
    },
}

impl PrintTextual for Instr {
    fn pp(&self) -> String {
        match self {
            Instr::Load {
                id,
                exp,
                typ: None,
                loc,
            } => format!("{} = load {}{}", id.pp(), exp.pp(), loc.pp()),
            Instr::Load {
                id,
                exp,
                typ: Some(typ),
                loc,
            } => format!("{}:{} = load {}{}", id.pp(), typ.pp(), exp.pp(), loc.pp()),
            Instr::Store {
                exp1,
                typ,
                exp2,
                loc,
            } => format!("store {} <- {}{}", exp1.pp(), exp2.pp(), loc.pp()),
            Instr::Prune { exp, loc } => todo!(),
            Instr::Let { id, exp, loc } => todo!(),
        }
    }
}
