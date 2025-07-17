/*
[OCaml Definitions]
module FieldDecl : sig
  type t = {qualified_name: qualified_fieldname; typ: Typ.t; attributes: Attr.t list}
end

type qualified_fieldname = {enclosing_class: TypeName.t; name: FieldName.t}
*/

use crate::textual_defs::{attr, fieldname, typ, typename};

#[derive(Debug)]
pub struct QualifiedFieldname {
    pub enclosing_class: typename::TypeName,
    pub name: fieldname::FieldName,
}

#[derive(Debug)]
pub struct FieldDecl {
    pub qualified_name: QualifiedFieldname,
    pub typ: typ::Typ,
    pub attributes: Vec<attr::Attr>,
}
