use crate::textual_defs::location;
/*
[OCaml Definition]
    module Attr : sig
    type t = {name: string; values: string list; loc: Location.t}
*/
pub struct T {
    name: String,
    values: Vec<String>,
    loc: location::T,
}
