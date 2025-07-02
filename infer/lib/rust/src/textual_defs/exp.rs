use crate::textual_defs::{
    attr::{self, Attr}, constant, fieldname::{self, FieldName}, ident, qualifiedprocname::{self, QualifiedProcName}, typ, typename::{self, TypeName}, varname::{self, VarName}, PrintTextual, PrintTextualWithSeperator
};

/*
[OCaml Definition]
module Exp : sig
  type call_kind = Virtual | NonVirtual [@@deriving equal]

  type t =
    | Var of Ident.t  (** pure variable: it is not an lvalue *)
    | Load of {exp: t; typ: Typ.t option}
    | Lvar of VarName.t  (** the address of a program variable *)
    | Field of {exp: t; field: qualified_fieldname}  (** field offset *)
    | Index of t * t  (** an array index offset: [exp1[exp2]] *)
    | Const of Const.t
    | Call of {proc: QualifiedProcName.t; args: t list; kind: call_kind}
    | Closure of
        { proc: QualifiedProcName.t
        ; captured: t list
        ; params: VarName.t list
        ; attributes: Attr.t list }
    | Apply of {closure: t; args: t list}
    | Typ of Typ.t
*/
#[derive(Debug)]
pub enum CallKind {
    Virtual,
    NonVirtual,
}

#[derive(Debug)]
pub struct QualifiedFieldname {
    pub enclosing_class: TypeName,
    pub name: FieldName,
}

#[derive(Debug)]
pub enum Exp {
    Var(ident::T),
    Load {
        exp: Box<Exp>,
        typ: Option<typ::Typ>,
    },
    LVar(VarName),
    Field {
        exp: Box<Exp>,
        field: QualifiedFieldname,
    },
    Index(Box<Exp>, Box<Exp>),
    Const(constant::Const),
    Call {
        proc: QualifiedProcName,
        args: Vec<Exp>,
        kind: CallKind,
    },
    Closure {
        proc: QualifiedProcName,
        captured: Vec<Exp>,
        params: Vec<VarName>,
        attributes: Vec<Attr>,
    },
    Apply {
        closure: Box<Exp>,
        args: Vec<Exp>,
    },
    Typ(typ::Typ),
}

impl PrintTextual for Exp {
    fn pp(&self) -> String {
        match self {
            Exp::Var(id) => id.pp(),
            Exp::Load { exp, typ } => todo!(),
            Exp::LVar(var_name) => format!("&{}", var_name.pp()),
            Exp::Field { exp, field } => todo!(),
            Exp::Index(exp, exp1) => todo!(),
            Exp::Const(t) => t.pp(),
            Exp::Call { proc, args, kind } => match kind {
                CallKind::Virtual => todo!(),
                CallKind::NonVirtual => format!("{}({})",proc.pp(),args.pp_list(",")),
            },
            Exp::Closure {
                proc,
                captured,
                params,
                attributes,
            } => todo!(),
            Exp::Apply { closure, args } => todo!(),
            Exp::Typ(typ) => todo!(),
        }
    }
}
