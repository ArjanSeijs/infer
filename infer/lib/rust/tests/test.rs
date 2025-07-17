#![feature(rustc_private)]
use std::io::prelude::*;
use std::fs::File;
use pretty_assertions::assert_eq;

use infer_rustc_mir::call_compiler;


fn read_file(sil_file: &str) -> String {
    let mut file = File::open(sil_file).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}

fn run_test(args: &mut Vec<String>, rust_file: &str, sil_file: &str) {    
    // Finally push filename
    args.push(rust_file.to_string());
    let result = call_compiler(&args);
    let expected = read_file(sil_file);
    match result {
        Ok(s) => assert_eq!(s, expected),
        Err(s) => assert!(false, "{}", s)
    };
}

fn default_args(args: &mut Vec<String>) {
    // 'executable'
    args.insert(0, "cargo".to_string());
    // Do no optimisation on test
    args.push("-Z".to_string());
    args.push("mir-opt-level=0".to_string());

    // Do not require main function
    args.push("--crate-type=lib".to_string());

    // Release Mode (No Alignment Checks)
    args.push("-C".to_string());
    args.push("opt-level=3".to_string());
    
    // Allow unused code.
    args.push("-A".to_string());
    args.push("dead_code".to_string());
    args.push("-A".to_string());
    args.push("unused_variables".to_string());
    args.push("-A".to_string());
    args.push("unused_must_use".to_string());
}

#[allow(unused)]
fn print_mir(args: &mut Vec<String>) {
    args.push("-Z".to_string());
    args.push("unpretty=mir".to_string());
}

#[test]
fn add0() {
    let args = &mut vec![];
    default_args(args);
    run_test(args, "./tests/programs/add0.rs","./tests/programs/add0.sil");
}

#[test]
fn call() {
    let args = &mut vec![];
    default_args(args);
    run_test(args, "./tests/programs/call.rs", "./tests/programs/call.sil");
}