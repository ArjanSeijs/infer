use crate::textual_defs::{
    attr, constant, fieldname, ident, qualifiedprocname, typ, typename, varname,
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
pub enum CallKind {
    Virtual,
    NonVirtual,
}

pub struct QualifiedFieldname {
    enclosing_class: typename::T,
    name: fieldname::T,
}

pub enum T {
    Var(ident::T),
    Load {
        exp: Box<T>,
        typ: Option<typ::T>,
    },
    LVar(varname::T),
    Field {
        exp: Box<T>,
        field: QualifiedFieldname,
    },
    Index(Box<T>, Box<T>),
    Const(constant::T),
    Call {
        proc: qualifiedprocname::T,
        args: Vec<T>,
        kind: CallKind,
    },
    Closure {
        proc: qualifiedprocname::T,
        captured: Vec<T>,
        params: Vec<varname::T>,
        attributes: Vec<attr::T>,
    },
    Apply {
        closure: Box<T>,
        args: Vec<T>,
    },
    Typ(typ::T),
}
