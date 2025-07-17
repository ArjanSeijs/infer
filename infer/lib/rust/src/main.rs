#![feature(rustc_private)]
use infer_rustc_mir::call_compiler;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    compiler_arguments(&mut args);
    let result = call_compiler(&args);
    match result {
        Ok(s) => println!("{}", s),
        Err(s) => println!("{}", s),
    }
}

fn compiler_arguments(args: &mut Vec<String>) {
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
}
