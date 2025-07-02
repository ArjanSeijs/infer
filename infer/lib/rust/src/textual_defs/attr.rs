use crate::textual_defs::location::Location;
/*
[OCaml Definition]
    module Attr : sig
    type t = {name: string; values: string list; loc: Location.t}
*/

#[derive(Debug)]
pub struct Attr {
    pub name: String,
    pub values: Vec<String>,
    pub loc: Location,
}
