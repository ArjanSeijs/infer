use stable_mir::ty::Span;

use crate::textual_defs::{name::{self, Name}, PrintTextual};

/*
[OCaml Definition]
module NodeName : NAME
*/

#[derive(Debug)]
pub struct NodeName {
    pub name: name::Name,
}

impl PrintTextual for NodeName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}

impl NodeName {
    pub fn new(value: String, span: Option<Span>) -> NodeName {
        NodeName {
            name: Name::new(value, span),
        }
    }
}
