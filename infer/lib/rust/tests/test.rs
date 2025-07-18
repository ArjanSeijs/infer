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
    fn constant0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/constant0.rs", "./tests/programs/constant/constant0.sil");
    }
    
    #[test]
    fn constant1() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/constant1.rs", "./tests/programs/constant/constant1.sil");
    }
    
    #[test]
    fn constant2() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/constant2.rs", "./tests/programs/constant/constant2.sil");
    }
    
    #[test]
    fn constant3() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/constant3.rs", "./tests/programs/constant/constant3.sil");
    }
    
    #[test]
    fn constant4() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/constant/constant4.rs", "./tests/programs/constant/constant4.sil");
    }
}

mod exp {
    use super::*;

    #[test]
    fn add0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/exp/add0.rs","./tests/programs/exp/add0.sil");
    }

    #[test]
    fn call() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/exp/call.rs", "./tests/programs/exp/call.sil");
    }
}

mod ident {
    use super::*;

    #[test]
    fn ident0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/ident/ident0.rs", "./tests/programs/ident/ident0.sil");
    }

    #[test]
    fn ident1() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/ident/ident1.rs", "./tests/programs/ident/ident1.sil");
    }
    
    #[test]
    fn ident2() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/ident/ident2.rs", "./tests/programs/ident/ident2.sil");
    }

    #[test]
    fn ident3() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/ident/ident3.rs", "./tests/programs/ident/ident3.sil");
    }

    #[test]
    fn ident4() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/ident/ident4.rs", "./tests/programs/ident/ident4.sil");
    }
}