// module Location : sig
//   type t = Known of {line: int; col: int} | Unknown [@@deriving compare]

use stable_mir::ty::Span;

use crate::textual_defs::PrintTextual;

#[derive(Debug,Clone)]
pub enum Location {
    Known { line: i64, loc: i64 },
    Unknown,
}

impl PrintTextual for Location {
    fn pp(&self) -> String {
        match self {
            Location::Known { line, loc } => format!("// {line}:{loc}"),
            Location::Unknown => format!(""),
        }
    }
}

impl Location {
    pub fn from_span(span: Option<Span>) -> Location {
        match span {
            Some(span) => {
                let lines = span.get_lines();
                Location::Known {
                    line: lines.start_line as i64,
                    loc: lines.start_col as i64,
                }
            }
            None => Location::Unknown,
        }
    }

    pub fn from_span_end(span: Option<Span>) -> Location {
        match span {
            Some(span) => {
                let lines = span.get_lines();
                Location::Known {
                    line: lines.end_line as i64,
                    loc: lines.end_col as i64,
                }
            }
            None => Location::Unknown,
        }
    }
}
