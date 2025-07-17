use crate::textual_defs::PrintTextual;

/*
[OCaml Definition]
module Const : sig
  type t =
    | Int of Z.t  (** integer constants *)
    | Null
    | Str of string  (** string constants *)
    | Float of float  (** float constants *)
end
*/
#[derive(Debug)]
pub enum Const {
    Int(i128),
    Null,
    Str(String),
    Float(f64),
}

impl PrintTextual for Const {
    fn pp(&self) -> String {
        match self {
            Const::Int(i) => format!("{i}"),
            Const::Null => format!("null"),
            Const::Str(str) => format!("\"{str}\""),
            Const::Float(f) => format!("{f}"),
        }
    }
}