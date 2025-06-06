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
pub enum T {
    Int(i128),
    Null,
    Str(String),
    Float(f64),
}
