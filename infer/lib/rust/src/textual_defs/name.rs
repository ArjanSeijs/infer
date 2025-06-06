use crate::textual_defs::location;

/*
[OCaml Definition]
    module type NAME = sig
    type t = {value: string; loc: Location.t} [@@deriving compare, equal, hash] 
*/
pub struct  T {
    value : String,
    loc: location::T
}