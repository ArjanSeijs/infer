use stable_mir::ty::Span;

use crate::textual_defs::{name::{self, Name}, PrintTextual};

/*
[OCaml Definition]
module ProcName : NAME 
*/

#[derive(Debug, Clone)]
pub struct ProcName {
    pub name: name::Name
}

impl PrintTextual for ProcName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}

impl ProcName {
    pub fn new(value: String, span: Option<Span>) -> ProcName {
        ProcName { name: Name::new(value, span) }
    }
}