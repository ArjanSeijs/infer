use crate::textual_defs::{exp, ident, location, typ};

/*
[OCaml Definition]
module Instr : sig
  type t =
    | Load of {id: Ident.t; exp: Exp.t; typ: Typ.t option; loc: Location.t}
        (** id <- *exp with *exp:typ *)
    | Store of {exp1: Exp.t; typ: Typ.t option; exp2: Exp.t; loc: Location.t}
        (** *exp1 <- exp2 with exp2:typ *)
    | Prune of {exp: Exp.t; loc: Location.t}  (** assume exp *)
    | Let of {id: Ident.t option; exp: Exp.t; loc: Location.t}  (** id = exp or _ = exp *)
  (* Remark that because Sil operations (add, mult...) are calls, we let the Textual programmer put
     expression in local variables, while SIL forbid that. The to_sil transformation will have to
     inline these definitions. *)

  val loc : t -> Location.t

  val pp : ?show_location:bool -> F.formatter -> t -> unit
end
*/
pub enum T {
    Load {
        id: ident::T,
        exp: exp::T,
        typ: Option<typ::T>,
        loc: location::T,
    },
    Store {
        exp1: exp::T,
        typ: Option<typ::T>,
        exp2: exp::T,
        loc: location::T,
    },
    Prune {
        exp: exp::T,
        loc: location::T,
    },
    Let {
        id: Option<ident::T>,
        exp: exp::T,
        loc: location::T,
    },
}
