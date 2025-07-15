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

use stable_mir::mir::LocalDecl;

use crate::textual_defs::{
    attr::Attr, qualifiedprocname::QualifiedProcName, typ::{Annotated}, PrintTextual, PrintTextualWithSeperator
};

#[derive(Debug)]
pub struct ProcDecl {
    pub qualified_name: QualifiedProcName,
    pub formal_types: Vec<Annotated>,
    pub result_type: Annotated,
    pub attributes: Vec<Attr>,
}

impl PrintTextual for ProcDecl {
    fn pp(&self) -> String {
        format!(
            "{}({}) : {}",
            self.qualified_name.pp(),
            self.formal_types.pp_list(","),
            self.result_type.pp()
        )
    }
}
