use crate::textual_defs::name;

/*
[OCaml Definition]
module BaseTypeName : NAME
*/
#[derive(Debug,Clone)]
pub struct BaseTypeName {
    pub name: name::Name
}