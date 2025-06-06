use crate::textual_defs::name;

/*
[OCaml Definition]
module VarName : sig
  (* variables names *)
  include NAME
*/
pub struct T {
    name: name::T
}