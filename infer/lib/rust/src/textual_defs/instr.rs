use stable_mir::{
    mir::{
        BinOp, ConstOperand, Operand, Place,
        Rvalue::{self, BinaryOp},
        Statement, StatementKind,
    },
    ty::{Allocation, ConstantKind::Allocated},
};

use crate::textual_defs::{
    PrintTextual, constant,
    exp::{self, Exp},
    ident, location, name, procname, qualifiedprocname, typ,
    varname::VarName,
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
        id: ident::Ident,
        exp: Exp,
        typ: Option<typ::Typ>,
        loc: location::Location,
    },
    Store {
        exp1: Exp,
        typ: Option<typ::Typ>,
        exp2: Exp,
        loc: location::Location,
    },
    Prune {
        exp: Exp,
        loc: location::Location,
    },
    Let {
        id: Option<ident::Ident>,
        exp: Exp,
        loc: location::Location,
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

pub fn statment_to_textual(statement: &Statement) -> Vec<Instr> {
    match &statement.kind {
        StatementKind::Assign(place, rvalue) => assign_statement_to_textual(place, rvalue),
        StatementKind::StorageLive(_) => vec![],
        StatementKind::StorageDead(_) => vec![],
        s => todo!("Statement to textual: {:?}", s),
    }
}

fn assign_statement_to_textual(place: &Place, rvalue: &Rvalue) -> Vec<Instr> {
    match rvalue {
        BinaryOp(bin_op, op1, op2) => assign_binop_to_textual(place, bin_op, op1, op2),
        s => todo!("{:?}", s),
    }
}

fn assign_binop_to_textual(
    place: &Place,
    bin_op: &BinOp,
    op1: &Operand,
    op2: &Operand,
) -> Vec<Instr> {
    let (op1, instr1) = operand_to_textual(op1);
    let (op2, instr2) = operand_to_textual(op2);

    let exp2 = Exp::Call {
        proc: qualifiedprocname::QualifiedProcName {
            enclosing_class: qualifiedprocname::EnclosingClass::TopLevel,
            name: procname::ProcName {
                name: name::T {
                    value: "__sil_plusa_int".to_string(),
                    loc: location::Location::Unknown,
                },
            },
        },
        args: vec![op1, op2],
        kind: exp::CallKind::NonVirtual,
    };

    let store = Instr::Store {
        exp1: Exp::LVar(VarName {
            name: name::T {
                value: format!("var_{}", place_as_int(place)),
                loc: location::Location::Unknown,
            },
        }),
        typ: Some(typ::Typ::Int),
        exp2,
        loc: location::Location::Unknown,
    };

    let mut instrs = vec![];
    if let Some(instr1) = instr1 {
        instrs.push(instr1);
    }
    if let Some(instr2) = instr2 {
        instrs.push(instr2);
    }
    instrs.push(store);
    instrs
}

fn operand_to_textual(operand: &Operand) -> (Exp, Option<Instr>) {
    // A constant should be an expression constant
    // A move or copy should be handled as a load instruction and reference to that variable
    match operand {
        Operand::Copy(place) | Operand::Move(place) => (
            Exp::Var(ident::Ident {
                val: place_as_int(place),
            }),
            Some(Instr::Load {
                id: ident::Ident {
                    val: place_as_int(place),
                },
                exp: Exp::LVar(VarName {
                    name: name::T {
                        value: format!("var_{}", place_as_int(place)),
                        loc: location::Location::Unknown,
                    },
                }),
                typ: Some(typ::Typ::Int),
                loc: location::Location::Unknown,
            }),
        ),
        Operand::Constant(const_operand) => {
            (Exp::Const(const_operand_to_textual(const_operand)), None)
        }
    }
}

fn const_operand_to_textual(const_operand: &ConstOperand) -> constant::Const {
    let const_ = const_operand.const_.kind();
    match const_ {
        Allocated(Allocation {
            bytes,
            provenance,
            align,
            mutability,
        }) => constant::Const::Int(bytes_to_int(bytes)),
        s => todo!("Const to textual: {:?}", s),
    }
}

fn place_as_int(place: &Place) -> i128 {
    place.local.try_into().unwrap()
}

fn bytes_to_int(bytes: &Vec<Option<u8>>) -> i128 {
    bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (b.unwrap_or_default() as i128) << i)
        .sum::<i128>()
}
