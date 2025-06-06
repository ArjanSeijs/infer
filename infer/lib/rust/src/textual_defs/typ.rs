/*
[OCaml Definition]
module Typ : sig
  type t =
    | Int  (** integer type *)
    | Float  (** float type *)
    | Null
    | Void  (** void type *)
    | Fun of function_prototype option  (** function type *)
    | Ptr of t  (** pointer type *)
    | Struct of TypeName.t  (** structured value type name *)
    | Array of t  (** array type *)
  [@@deriving equal]

  and function_prototype = {params_type: t list; return_type: t} [@@deriving equal]

  val pp : F.formatter -> t -> unit

  type annotated = {typ: t; attributes: Attr.t list}

  val is_annotated : f:(Attr.t -> bool) -> annotated -> bool

  val pp_annotated : F.formatter -> annotated -> unit

  val mk_without_attributes : t -> annotated
end
 */

use crate::textual_defs::attr;

pub enum T {
    Int,
    Float,
    Null,
    Fun,
    Ptr,
    Struct,
    Array,
}

pub struct FunctionPrototype {
  params_type : Vec<T>,
  return_type: T
}

pub struct Annotated {
  typ: T,
  attributes: Vec<attr::T>
}
