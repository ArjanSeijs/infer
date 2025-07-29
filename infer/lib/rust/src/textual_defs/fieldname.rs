use crate::textual_defs::{name, PrintTextual};

/*
[OCaml Definition]
    module FieldName : NAME 
*/

#[derive(Debug)]
pub struct FieldName {
    pub name: name::Name
}

impl PrintTextual for FieldName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}
