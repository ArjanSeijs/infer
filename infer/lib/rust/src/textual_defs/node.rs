use stable_mir::mir::BasicBlock;

use crate::textual_defs::{
    ident::Ident, instr::Instr, location::Location, nodename::NodeName, terminator::Terminator, typ::Typ, PrintTextual
};

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

#[derive(Debug)]
pub struct Node {
    pub label: NodeName,
    pub ssa_parameters: Vec<(Ident, Typ)>,
    pub exn_succs: Vec<NodeName>,
    pub last: Terminator,
    pub instrs: Vec<Instr>,
    pub last_loc: Location,
    pub label_loc: Location,
}
impl PrintTextual for Node {
    fn pp(&self) -> String {
        let node_name = format!("  #{}: ", self.label.name.value);
        let instrs: Vec<_> = self
            .instrs
            .iter()
            .map(|instr| format!("    {}", instr.pp()))
            .collect();
        let terminator = self.last.pp();
        format!("{node_name}\n{}\n    {terminator}\n",instrs.join("\n"))
    }
}
