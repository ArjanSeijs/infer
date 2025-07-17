use crate::textual_defs::basetypename::BaseTypeName;

/*
[OCaml Definition]
module TypeName : sig
  (* structured value type name *)
  type t = {name: BaseTypeName.t; args: t list} [@@deriving compare, equal, hash]
*/

#[derive(Debug,Clone)]
pub struct TypeName {
    name: BaseTypeName,
    args: Vec<TypeName>,
}
