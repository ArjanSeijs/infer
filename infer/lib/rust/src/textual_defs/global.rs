use crate::textual_defs::{attr, typ, varname};

/*
[OCaml Definition]
module Global : sig
  type t = {name: VarName.t; typ: Typ.t; attributes: Attr.t list}
end
*/

#[derive(Debug)]
pub struct Global {
    pub name: varname::VarName,
    pub typ: typ::Typ,
    pub attributes: Vec<attr::Attr>
}