use crate::textual_defs::{name, PrintTextual};

/*
[OCaml Definition]
module BaseTypeName : NAME
*/
#[derive(Debug,Clone)]
pub struct BaseTypeName {
    pub name: name::Name
}

impl BaseTypeName {
    pub fn from_string(name: String) -> Self {
        BaseTypeName {
            name: name::Name::new(name, None),
        }
    }
}

impl PrintTextual for BaseTypeName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}
