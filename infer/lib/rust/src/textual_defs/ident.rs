/*
[OCaml Definition]
    module Ident : sig
    type t [@@deriving equal]
*/

use crate::textual_defs::PrintTextual;

#[derive(Debug)]
pub struct Ident {
    pub val: i128,
}

impl PrintTextual for Ident {
    fn pp(&self) -> String {
        format!("n{}", self.val)
    }
}
