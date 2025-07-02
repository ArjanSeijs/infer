use crate::textual_defs::location::span_to_location;
use crate::textual_defs::procdesc::item_to_procdesc;
use crate::textual_defs::{PrintTextual, name};
use stable_mir::mir::VarDebugInfoContents;
use stable_mir::CrateItems;
use stable_mir::ty::Span;

pub fn mir_to_textual(items: CrateItems) -> String {
    let header = "// \n\
    // Copyright (c) Facebook, Inc. and its affiliates. \n\
    // \n\
    // This source code is licensed under the MIT license found in the \n\
    // LICENSE file in the root directory of this source tree. \n\
    \n\
    .source_language = \"rust\"\n\n";
    let mut sil_code = String::new();
    sil_code.push_str(header);
    for item in items {
        let translation = item_to_procdesc(&item).pp();
        for var_debug_info in &item.expect_body().var_debug_info {
            sil_code.push_str(&print_debug(var_debug_info));
        }
        sil_code.push_str(&translation);
    }
    sil_code
}

fn print_debug(var_debug_info: &stable_mir::mir::VarDebugInfo) -> String {
    let location = match &var_debug_info.value {
        VarDebugInfoContents::Place(place) => format!("_{}", place.local),
        VarDebugInfoContents::Const(_) => format!("const"),
    };
    format!("// debug {} => {}\n", var_debug_info.name, location)
}

pub fn parse_id(i: usize, span: Span) -> name::T {
    let name = format!("var_{i}");
    name::T {
        value: name,
        loc: span_to_location(span),
    }
}
