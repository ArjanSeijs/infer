use crate::textual_defs::name;

/*
[OCaml Definition]
module BaseTypeName : NAME
*/
#[derive(Debug)]
pub struct BaseTypeName {
    pub name: name::T
}