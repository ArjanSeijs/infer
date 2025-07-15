#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

#[macro_use]
extern crate rustc_smir;
extern crate stable_mir;
use std::{env, ops::ControlFlow};

mod textual;
#[allow(unused)]
mod textual_defs;
use stable_mir::CompilerError;
use textual::mir_to_textual;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    compiler_arguments(&mut args);
    call_compiler(args);
}

pub fn call_compiler(args: Vec<String>) {
    let analyze_code = || -> ControlFlow<Result<String, String>, String> {
        let result = std::panic::catch_unwind(|| {
            mir_to_textual(stable_mir::all_local_items())
        });
        match result {
            Ok(translation) => ControlFlow::Break(Ok(translation)),
            Err(e) => ControlFlow::Break(Err(format!("{:?}", e))),
        }
    };
    let result = run!(&args, analyze_code);
    match result {
        Ok(ok) => println!("{}", ok),
        Err(CompilerError::Interrupted(Ok(s))) => println!("{}", s),
        Err(CompilerError::Interrupted(Err(s))) => println!("// Error {}", s),
        Err(err) => println!("// CompileError : {:?}", err),
    }
}

fn compiler_arguments(args: &mut Vec<String>) {

    let mut i = 0;
    let mut ins = || {
        i += 1;
        i
    };
    // Release Mode (No Alignment Checks)
    args.insert(1, "-C".to_string());
    args.insert(2, "opt-level=3".to_string());
    // Allow unused code.
    args.insert(3, "-A".to_string());
    args.insert(4, "dead_code".to_string());
    args.insert(5, "-A".to_string());
    args.insert(6, "unused_variables".to_string());
    args.insert(7, "-A".to_string());
    args.insert(8, "unused_must_use".to_string());
    
    args.insert(ins(), "--crate-type=lib".to_string());
    // args.insert(ins(), "-Z".to_string());
    // args.insert(ins(), "unpretty=mir".to_string());
}
