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
    PrintTextual, PrintTextualWithSeperator,
    attr::Attr,
    qualifiedprocname::{QualifiedProcName, to_qualified_name},
    typ::{self, local_decl_to_type},
};

#[derive(Debug)]
pub struct ProcDecl {
    pub qualified_name: QualifiedProcName,
    pub formal_types: Vec<typ::Annotated>,
    pub result_type: typ::Annotated,
    pub attributes: Vec<Attr>,
}

pub fn to_proc_decl(name: &String, arg_locals: &[LocalDecl], ret_local: &LocalDecl) -> ProcDecl {
    let attributes = vec![];
    let result_type = local_decl_to_type(ret_local);
    let qualified_name = to_qualified_name(name);
    let formal_types = arg_locals
        .iter()
        .map(|decl| local_decl_to_type(decl))
        .collect();
    ProcDecl {
        qualified_name,
        formal_types,
        result_type,
        attributes,
    }
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
