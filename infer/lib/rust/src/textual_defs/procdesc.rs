use crate::textual_defs::{location, node, nodename, procdecl, typ, varname};

/*
[OCaml Defintion]
module ProcDesc : sig
  type t =
    { procdecl: ProcDecl.t
    ; nodes: Node.t list
    ; start: NodeName.t
    ; params: VarName.t list
    ; locals: (VarName.t * Typ.annotated) list
    ; exit_loc: Location.t }
*/
pub struct  T {
    proc_decl: procdecl::T,
    nodes: Vec<node::T>,
    start: nodename::T,
    params: Vec<varname::T>,
    locals: Vec<(varname::T, typ::Annotated)>,
    exit_loc: location::T
}