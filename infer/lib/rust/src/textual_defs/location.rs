// module Location : sig
//   type t = Known of {line: int; col: int} | Unknown [@@deriving compare]

use stable_mir::ty::Span;

use crate::textual_defs::PrintTextual;

#[derive(Debug)]
pub enum Location {
    Known { line: i64, loc: i64 },
    Unknown
}

pub fn span_to_location(span : Span) -> Location {
    let lines = span.get_lines();
    Location::Known { line: lines.start_line as i64, loc: lines.start_col as i64 }
}

impl PrintTextual for Location {
    fn pp(&self) -> String {
        match self {
            Location::Known { line, loc } => format!("// {line}:{loc}"),
            Location::Unknown => format!(""),
        }
    }
}