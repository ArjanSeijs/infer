use stable_mir::mir::BasicBlock;

use crate::textual_defs::{
    PrintTextual, ident,
    instr::{self, statment_to_textual},
    location, name, node,
    nodename::{self, NodeName},
    terminator::{self, terminator_to_textual},
    typ,
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
    pub label: nodename::NodeName,
    pub ssa_parameters: Vec<(ident::Ident, typ::Typ)>,
    pub exn_succs: Vec<nodename::NodeName>,
    pub last: terminator::Terminator,
    pub instrs: Vec<instr::Instr>,
    pub last_loc: location::Location,
    pub label_loc: location::Location,
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

pub fn block_to_textual(block: &BasicBlock) -> Node {
    let statements = &block.statements;
    let terminator = &block.terminator;

    let label = NodeName {
        name: name::T {
            value: "todo".to_string(),
            loc: location::Location::Unknown,
        },
    };
    let ssa_parameters = vec![];
    let exn_succs = vec![];
    let last = terminator_to_textual(terminator);
    let instrs = statements.iter().flat_map(statment_to_textual).collect();
    let last_loc = location::Location::Unknown;
    let label_loc = location::Location::Unknown;
    node::Node {
        label,
        ssa_parameters,
        exn_succs,
        last,
        instrs,
        last_loc,
        label_loc,
    }
}
