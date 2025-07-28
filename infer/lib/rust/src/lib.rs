#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

#[macro_use]
extern crate rustc_smir;
extern crate stable_mir;
use std::ops::ControlFlow;

pub mod textual;
pub mod utils;
#[allow(unused)]
mod textual_defs;
use crate::textual::mir_to_textual;
use stable_mir::CompilerError;

pub fn call_compiler(args: &Vec<String>) -> Result<String, String> {
    let analyze_code = || -> ControlFlow<Result<String, _>, _> {
        let result = std::panic::catch_unwind(|| mir_to_textual(stable_mir::all_local_items()));
        match result {
            Ok(translation) => ControlFlow::Break(Ok(translation)),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    };
    let result = run!(&args, analyze_code);
    match result {
        Ok(ok) => Ok(ok),
        Err(CompilerError::Interrupted(Ok(s))) => Ok(s),
        Err(CompilerError::Interrupted(Err(s))) => Err(format!("// Error {:?}", s)),
        Err(err) => Err(format!("// CompileError : {:?}", err)),
    }
}
