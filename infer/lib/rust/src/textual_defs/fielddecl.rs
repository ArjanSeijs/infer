/*
[OCaml Definitions]
module FieldDecl : sig
  type t = {qualified_name: qualified_fieldname; typ: Typ.t; attributes: Attr.t list}
end

type qualified_fieldname = {enclosing_class: TypeName.t; name: FieldName.t}
*/

use crate::textual_defs::{attr, fieldname, typ, typename};

pub struct QualifiedFieldname {
    enclosing_class: typename::T,
    name: fieldname::T,
}
pub struct T {
    qualified_name: QualifiedFieldname,
    typ: typ::T,
    attributes: Vec<attr::T>,
}
