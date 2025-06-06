// module Location : sig
//   type t = Known of {line: int; col: int} | Unknown [@@deriving compare]
pub enum T {
    Known { line: i64, loc: i64 },
    Unknown
}
