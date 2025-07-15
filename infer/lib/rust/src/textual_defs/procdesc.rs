/*
[OCaml Defintion]
module ProcDesc : sig
  type t =
    { procdecl: ProcDecl.t
    ; nodes: Node.t list
    ; start: NodeName.t
    ; params: VarName.t list
    ; locals: (VarName.t * Typ.annotated) list
    ; exit_loc: Location.t }
*/

use std::{collections::HashMap, iter::zip};

use rustc_middle::mir::BasicBlock;
use stable_mir::{CrateDef, CrateItem};

use crate::textual_defs::{
    location::Location, node::Node, nodename::NodeName, procdecl::ProcDecl, typ::Annotated, varname::VarName, PrintTextual
};

#[derive(Debug)]
pub struct ProcDesc {
    pub proc_decl: ProcDecl,
    pub nodes: Vec<Node>,
    pub start: NodeName,
    pub params: Vec<VarName>,
    pub locals: Vec<(VarName, Annotated)>,
    pub exit_loc: Location,
}

impl PrintTextual for ProcDesc {
    fn pp(&self) -> String {
        let signature = self.signature_string();
        let signature = format!("define {} {{\n", signature);
        let locals = if self.locals.is_empty() {
            "".to_string()
        } else {
            let locals: Vec<String> = self
                .locals
                .iter()
                .map(|(v, a)| format!("{}: {}", v.name.value, a.pp()))
                .collect();
            let locals = locals.join(", ");
            format!("  local {locals}\n")
        };
        let nodes: Vec<String> = self.nodes.iter().map(|n| n.pp()).collect();
        let nodes = nodes.join("\n");
        format!("{signature}{locals}{nodes}}}\n")
    }
}

impl ProcDesc {
    fn signature_string(&self) -> String {
        let procdecl = &self.proc_decl;
        let params = &self.params;
        let formals = &self.proc_decl.formal_types;
        let args = zip(formals, params);

        let args: Vec<String> = args
            .map(|(annotated, varname)| format!("{}:{}", varname.pp(), annotated.pp()))
            .collect();
        format!(
            "{}({}) : {}",
            procdecl.qualified_name.pp(),
            args.join(","),
            procdecl.result_type.pp()
        )
    }
}

// pub fn item_to_procdesc(item: &CrateItem) -> ProcDesc {
//     let body = item.expect_body();
//     let def = item.def_id();
//     let name = &def.name();

//     let arg_locals = body.arg_locals();
//     let ret_local = body.ret_local();
//     let inner_locals = body.inner_locals();
//     let blocks = &body.blocks;

//         print_debug(item, name, arg_locals, ret_local, inner_locals, blocks);

//         let proc_decl = to_proc_decl(name, arg_locals, ret_local);
//         let nodes = blocks.iter().map(block_to_textual).collect();
//         let start = crate::textual_defs::nodename::NodeName {
//             name: name::Name {
//                 value: "#entry".to_string(),
//                 loc: Location::Unknown,
//             },
//         };

//         // In mir the declerations are layed out as follows:
//         // 0: ret_local, the return type is always located on 0
//         // 1 ... #args : arg_locals, the next variables are the arguments provided to the function
//         // #args ... : inner_locals, the following variables are
//         let params: Vec<VarName> = arg_locals
//             .iter()
//             .enumerate()
//             .map(|(i, decl)| parse_id(i + 1, decl.span))
//             .map(|s| VarName { name: s })
//             .collect();

//         let mut locals: Vec<(VarName, typ::Annotated)> = inner_locals
//             .iter()
//             .enumerate()
//             .map(|(i, decl)| {
//                 (
//                     VarName {
//                         name: parse_id(i + 1 + params.len(), decl.span),
//                     },
//                     local_decl_to_type(decl),
//                 )
//             })
//             .collect();
//         locals.push(
//             (VarName {
//                 name: name::Name {
//                     value: "var_0".to_string(),
//                     loc: Location::Unknown,
//                 },
//             },typ::Annotated { typ: typ::Typ::Int, attributes: vec![] }),
//         );
//         let exit_loc = location::Location::Unknown;
//         let proc_desc = ProcDesc {
//             proc_decl,
//             nodes,
//             start,
//             params,
//             locals,
//             exit_loc,
//         };

//         proc_desc
// }

// fn print_debug(
//     item: &CrateItem,
//     name: &String,
//     arg_locals: &[stable_mir::mir::LocalDecl],
//     ret_local: &stable_mir::mir::LocalDecl,
//     inner_locals: &[stable_mir::mir::LocalDecl],
//     blocks: &Vec<stable_mir::mir::BasicBlock>,
// ) {
//     let x = stable_mir::local_crate();
//     dbg!("============");
//     dbg!(item);
//     dbg!(name);
//     dbg!(arg_locals);
//     dbg!(ret_local);
//     dbg!(inner_locals);
//     dbg!(blocks);
//     dbg!(x.fn_defs());
// }
