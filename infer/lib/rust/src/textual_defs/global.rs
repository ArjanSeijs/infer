use crate::textual_defs::{attr, typ, varname};

/*
[OCaml Definition]
module Global : sig
  type t = {name: VarName.t; typ: Typ.t; attributes: Attr.t list}
end
*/
pub struct T {
    name: varname::T,
    typ: typ::T,
    attributes: Vec<attr::T>
}