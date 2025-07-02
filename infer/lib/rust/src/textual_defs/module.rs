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

#[derive(Debug)]
pub enum Decl {
    Global(global::Global),
    Struct(struct_::Struct),
    Procdecl(procdecl::ProcDecl),
    Proc(procdesc::ProcDesc)
}

#[derive(Debug)]
pub struct T {
    attrs: Vec<attr::Attr>,
    decls: Vec<Decl>,
    sourcefile : sourcefile::SourceFile
}