use crate::textual_defs::{PrintTextual, location::Location, name, procname, typename};

/*
[OCAML Definitions]
type enclosing_class = TopLevel | Enclosing of TypeName.t
type t = {enclosing_class: enclosing_class; name: ProcName.t} [@@deriving compare, equal, hash]
*/

#[derive(Debug)]
pub enum EnclosingClass {
    TopLevel,
    Enclosing(typename::TypeName),
}

#[derive(Debug)]
pub struct QualifiedProcName {
    pub enclosing_class: EnclosingClass,
    pub name: procname::ProcName,
}

impl PrintTextual for QualifiedProcName {
    fn pp(&self) -> String {
        //TODO expand for enclosingclass
        format!("{}", self.name.pp())
    }
}

pub fn to_qualified_name(name: &String) -> QualifiedProcName {
    let enclosing_class = EnclosingClass::TopLevel;
    let name = {
        let value = name.clone();
        let loc = Location::Unknown;
        name::T { value, loc }
    };
    QualifiedProcName {
        enclosing_class,
        name: procname::ProcName { name },
    }
}
