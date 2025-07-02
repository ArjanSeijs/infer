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
use textual::mir_to_textual;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.insert(1, "-C".to_string());
    args.insert(2, "opt-level=3".to_string());
    // args.insert(3, "-Z".to_string());
    // args.insert(4, "unpretty=mir".to_string());
    
    let analyze_code = || -> ControlFlow<Result<String,String>, String> {
        let translation = mir_to_textual(stable_mir::all_local_items());
        ControlFlow::Break(Ok(translation))
    };
    let result = run!(&args, analyze_code);
    match result {
        Ok(ok) => println!("{}", ok),
        Err(stable_mir::CompilerError::Interrupted(Ok(s))) => println!("{}",s),
        Err(stable_mir::CompilerError::Interrupted(Err(s))) => println!("// Error {}",s),
        Err(err) => println!("// Error : {:?}", err),
    }
}
