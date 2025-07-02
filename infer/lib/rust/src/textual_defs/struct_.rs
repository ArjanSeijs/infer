use crate::textual_defs::{attr, typename, fielddecl};

/*
[OCaml Definition]
module Struct : sig
  type t =
    {name: TypeName.t; supers: TypeName.t list; fields: FieldDecl.t list; attributes: Attr.t list}
*/

#[derive(Debug)]
pub struct Struct {
    pub name: typename::TypeName,
    pub supers: Vec<typename::TypeName>,
    pub fields: Vec<fielddecl::FieldDecl>,
    pub attributes: Vec<attr::Attr>
}