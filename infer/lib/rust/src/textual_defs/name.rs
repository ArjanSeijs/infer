use crate::textual_defs::{location, PrintTextual};

/*
[OCaml Definition]
    module type NAME = sig
    type t = {value: string; loc: Location.t} [@@deriving compare, equal, hash] 
*/

#[derive(Debug)]
pub struct  T {
    pub value : String,
    pub loc: location::Location
}

impl PrintTextual for T {
    fn pp(&self) -> String {
        self.value.clone()
    }
}