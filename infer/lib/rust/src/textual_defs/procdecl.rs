/*
module ProcDecl : sig
  type t =
    { qualified_name: QualifiedProcName.t
    ; formals_types: Typ.annotated list option
          (** The list of formal argument types may be unknown. Currently, it is possible only for
              external function declarations when translating from Hack and is denoted with a
              special [...] syntax. Functions defined within a textual module always have a fully
              declared list of formal parameters. *)
    ; result_type: Typ.annotated
    ; attributes: Attr.t list }
*/

use crate::textual_defs::{attr, qualifiedprocname, typ};

pub struct  T {
    qualified_name: qualifiedprocname::T,
    formal_types: typ::Annotated,
    result_type: typ::Annotated,
    attributes: Vec<attr::T>
}