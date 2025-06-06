use crate::textual_defs::basetypename;

/*
[OCaml Definition]
module TypeName : sig
  (* structured value type name *)
  type t = {name: BaseTypeName.t; args: t list} [@@deriving compare, equal, hash]
*/
pub struct T {
    name: basetypename::T,
    args: Vec<T>,
}
