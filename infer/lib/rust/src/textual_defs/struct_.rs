use crate::textual_defs::{attr, typename, fielddecl};

/*
[OCaml Definition]
module Struct : sig
  type t =
    {name: TypeName.t; supers: TypeName.t list; fields: FieldDecl.t list; attributes: Attr.t list}
*/
pub struct T {
    name: typename::T,
    supers: Vec<typename::T>,
    fields: Vec<fielddecl::T>,
    attributes: Vec<attr::T>
}