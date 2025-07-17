use std::collections::HashMap;

use stable_mir::{mir::Place, ty::Span};

use crate::textual_defs::{
    PrintTextual,
    name::{self, Name},
    typ::Typ,
};

/*
[OCaml Definition]
module VarName : sig
  (* variables names *)
  include NAME
*/

#[derive(Debug)]
pub struct VarName {
    pub(crate) name: name::Name,
}

impl PrintTextual for VarName {
    fn pp(&self) -> String {
        self.name.pp()
    }
}

impl VarName {
    pub fn new(value: String, span: Option<Span>) -> VarName {
        VarName {
            name: Name::new(value, span),
        }
    }

    pub fn from_place(place: &Place, place_map: &HashMap<usize, (String, Typ)>) -> VarName {
        VarName::from_index(place.local, place_map)
    }

    pub fn from_index(index : usize, place_map: &HashMap<usize, (String, Typ)>) -> VarName {
        let (id, _) = place_map.get(&index).unwrap();
        VarName {
            name: Name::new(id.clone(), None),
        }
    }
}
