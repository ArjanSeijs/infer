use stable_mir::ty::Span;

use crate::textual_defs::location::Location;
use crate::textual_defs::PrintTextual;

/*
[OCaml Definition]
    module type NAME = sig
    type t = {value: string; loc: Location.t} [@@deriving compare, equal, hash] 
*/

#[derive(Debug, Clone)]
pub struct  Name {
    pub value : String,
    pub loc: Location
}

impl PrintTextual for Name {
    fn pp(&self) -> String {
        self.value.clone()
    }
}

impl Name {
    pub fn new(value : String, span : Option<Span>) -> Name {
        Name {
            value,
            loc: Location::from_span(span)
        }
    }
}