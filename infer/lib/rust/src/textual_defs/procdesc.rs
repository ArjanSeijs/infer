use std::{io::Write, iter::zip};

use stable_mir::{CrateDef, CrateItem, CrateItems};

use crate::{
    textual::parse_id,
    textual_defs::{
        PrintTextual, WriteTextual,
        location::{self, Location},
        name,
        node::{self, block_to_textual},
        nodename,
        procdecl::{self, to_proc_decl},
        procdesc,
        typ::{self, kind_to_textual, local_decl_to_type},
        varname::{self, VarName},
    },
};

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

#[derive(Debug)]
pub struct ProcDesc {
    pub proc_decl: procdecl::ProcDecl,
    pub nodes: Vec<node::Node>,
    pub start: nodename::NodeName,
    pub params: Vec<varname::VarName>,
    pub locals: Vec<(varname::VarName, typ::Annotated)>,
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

pub fn item_to_procdesc(item: &CrateItem) -> procdesc::ProcDesc {
    let body = item.expect_body();
    let def = item.def_id();
    let name = &def.name();

    let arg_locals = body.arg_locals();
    let ret_local = body.ret_local();
    let inner_locals = body.inner_locals();
    let blocks = &body.blocks;

    print_debug(item, name, arg_locals, ret_local, inner_locals, blocks);

    let proc_decl = to_proc_decl(name, arg_locals, ret_local);
    let nodes = blocks.iter().map(block_to_textual).collect();
    let start = crate::textual_defs::nodename::NodeName {
        name: name::T {
            value: "#entry".to_string(),
            loc: location::Location::Unknown,
        },
    };

    // In mir the declerations are layed out as follows:
    // 0: ret_local, the return type is always located on 0
    // 1 ... #args : arg_locals, the next variables are the arguments provided to the function
    // #args ... : inner_locals, the following variables are
    let params: Vec<VarName> = arg_locals
        .iter()
        .enumerate()
        .map(|(i, decl)| parse_id(i + 1, decl.span))
        .map(|s| VarName { name: s })
        .collect();

    let mut locals: Vec<(VarName, typ::Annotated)> = inner_locals
        .iter()
        .enumerate()
        .map(|(i, decl)| {
            (
                VarName {
                    name: parse_id(i + 1 + params.len(), decl.span),
                },
                local_decl_to_type(decl),
            )
        })
        .collect();
    locals.push(
        (VarName {
            name: name::T {
                value: "var_0".to_string(),
                loc: Location::Unknown,
            },
        },typ::Annotated { typ: typ::Typ::Int, attributes: vec![] }),
    );
    let exit_loc = location::Location::Unknown;
    let proc_desc = procdesc::ProcDesc {
        proc_decl,
        nodes,
        start,
        params,
        locals,
        exit_loc,
    };

    proc_desc
}

fn print_debug(
    item: &CrateItem,
    name: &String,
    arg_locals: &[stable_mir::mir::LocalDecl],
    ret_local: &stable_mir::mir::LocalDecl,
    inner_locals: &[stable_mir::mir::LocalDecl],
    blocks: &Vec<stable_mir::mir::BasicBlock>,
) {
    let x = stable_mir::local_crate();
    dbg!("============");
    dbg!(item);
    dbg!(name);
    dbg!(arg_locals);
    dbg!(ret_local);
    dbg!(inner_locals);
    dbg!(blocks);
    dbg!(x.fn_defs());
}
