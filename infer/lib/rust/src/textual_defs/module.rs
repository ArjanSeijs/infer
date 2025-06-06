use crate::textual_defs::{attr, struct_, global, procdecl, procdesc, sourcefile};

/*
[OCaml Definition]
module Module = struct
  type decl =
    | Global of Global.t
    | Struct of Struct.t
    | Procdecl of ProcDecl.t
    | Proc of ProcDesc.t

  type t = {attrs: Attr.t list; decls: decl list; sourcefile: SourceFile.t}
*/
pub enum Decl {
    Global(global::T),
    Struct(struct_::T),
    Procdecl(procdecl::T),
    Proc(procdesc::T)
}

pub struct T {
    attrs: Vec<attr::T>,
    decls: Vec<Decl>,
    sourcefile : sourcefile::T
}