use crate::textual_defs::{name, PrintTextual};

/*
[OCaml Definition]
module ProcName : NAME 
*/

#[derive(Debug)]
pub struct ProcName {
    pub name: name::T
}

impl PrintTextual for ProcName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}