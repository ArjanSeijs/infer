use crate::textual_defs::{attr::{self, Attr}, fielddecl::{self, FieldDecl}, typename::{self, TypeName}};

/*
[OCaml Definition]
module Struct : sig
  type t =
    {name: TypeName.t; supers: TypeName.t list; fields: FieldDecl.t list; attributes: Attr.t list}
*/

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub supers: Vec<TypeName>,
    pub fields: Vec<FieldDecl>,
    pub attributes: Vec<Attr>
}