/*
[OCaml Definition]
    module Ident : sig
    type t [@@deriving equal]
*/

use crate::textual_defs::PrintTextual;

#[derive(Debug)]
pub struct T {
    pub val: i128,
}

impl PrintTextual for T {
    fn pp(&self) -> String {
        format!("n{}", self.val)
    }
}
