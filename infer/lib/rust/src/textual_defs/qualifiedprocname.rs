use crate::textual_defs::{procname, typename};

/*
[OCAML Definitions]
type enclosing_class = TopLevel | Enclosing of TypeName.t
type t = {enclosing_class: enclosing_class; name: ProcName.t} [@@deriving compare, equal, hash]
*/
pub enum EnclosingClass {
    TopLevel,
    Enclosing(typename::T)

}

pub struct  T {
    enclosing_class: EnclosingClass,
    name : procname::T
}