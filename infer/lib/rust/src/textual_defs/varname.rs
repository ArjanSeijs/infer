use crate::textual_defs::{name, PrintTextual};

/*
[OCaml Definition]
module VarName : sig
  (* variables names *)
  include NAME
*/

#[derive(Debug)]
pub struct VarName {
    pub(crate) name: name::T
}

impl PrintTextual for VarName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}