use crate::textual_defs::name;

/*
[OCaml Definition]
module NodeName : NAME 
*/

#[derive(Debug)]
pub struct NodeName {
    pub name: name::T
}