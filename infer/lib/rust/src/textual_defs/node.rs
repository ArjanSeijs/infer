use crate::textual_defs::{ident, instr, location, node, nodename, terminator, typ};

/*
[OCaml Definition]
module Node : sig
  type t =
    { label: NodeName.t
    ; ssa_parameters: (Ident.t * Typ.t) list
    ; exn_succs: NodeName.t list  (** successor exception nodes *)
    ; last: Terminator.t
    ; instrs: Instr.t list
    ; last_loc: Location.t  (** location of last instruction in file *)
    ; label_loc: Location.t  (** location of label in file *) }
  [@@deriving equal]

  module Set : Stdlib.Set.S with type elt = t
end
*/
pub struct T {
    label: nodename::T,
    ssa_parameters: Vec<(ident::T, typ::T)>,
    exn_succs: Vec<nodename::T>,
    last: terminator::T,
    instrs: Vec<instr::T>,
    last_loc: location::T,
    label_loc: location::T
}