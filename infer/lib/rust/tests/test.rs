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

mod constant {
    use super::*;

    #[test]
    fn literal_float() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/literal_float.rs", "./tests/programs/constant/literal_float.sil");
    }

    #[test]
    fn literal_int() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/literal_int.rs", "./tests/programs/constant/literal_int.sil");
    }

    #[test]
    fn literal_mixed() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/literal_mixed.rs", "./tests/programs/constant/literal_mixed.sil");
    }

    #[test]
    fn literal_str() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/literal_str.rs", "./tests/programs/constant/literal_str.sil");
    }
    
    #[test]
    fn literal_unit() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/literal_unit.rs", "./tests/programs/constant/literal_unit.sil");
    }    
}

mod exp {
    use super::*;
    
    #[test]
    fn add0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/exp/add0.rs", "./tests/programs/exp/add0.sil");
    }

    #[test]
    fn call() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/exp/call.rs", "./tests/programs/exp/call.sil");
    }
}
