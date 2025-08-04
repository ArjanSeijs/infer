use stable_mir::ty::Span;

use crate::textual_defs::{location::Location, name::{self, Name}, procname::{self, ProcName}, typename, PrintTextual};

/*
[OCAML Definitions]
type enclosing_class = TopLevel | Enclosing of TypeName.t
type t = {enclosing_class: enclosing_class; name: ProcName.t} [@@deriving compare, equal, hash]
*/

#[derive(Debug, Clone)]
pub enum EnclosingClass {
    TopLevel,
    Enclosing(typename::TypeName),
}

#[derive(Debug, Clone)]
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

impl QualifiedProcName {
    pub fn new(value : String, span : Option<Span>) -> QualifiedProcName {
        QualifiedProcName {
            name: ProcName::new(value, span),
            enclosing_class: EnclosingClass::TopLevel
        }
    }
}
