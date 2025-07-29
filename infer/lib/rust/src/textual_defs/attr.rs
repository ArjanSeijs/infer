use crate::textual_defs::{PrintTextual, location::Location};
/*
[OCaml Definition]
    module Attr : sig
    type t = {name: string; values: string list; loc: Location.t}
*/

#[derive(Debug, Clone)]
pub struct Attr {
    pub name: String,
    pub values: Vec<String>,
    pub loc: Location,
}

impl PrintTextual for Attr {
    fn pp(&self) -> String {
        if self.values.is_empty() {
            format!(".{}", self.name)
        } else {
            let joined = self.values.join(", ");
            format!(".{} = \"{}\"", self.name, joined)
        }
    }
}
