use crate::textual_defs::name;

/*
[OCaml Definition]
    module FieldName : NAME 
*/

#[derive(Debug)]
pub struct FieldName {
    pub name: name::Name
}