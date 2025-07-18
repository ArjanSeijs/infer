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

mod arithmetic {
    use super::*;

    #[test]
    fn add0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/arithmetic/add0.rs", "./tests/programs/arithmetic/add0.sil");
    }
}

mod assignment {
    use super::*;

    #[test]
    fn arithmetic0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/assignment/arithmetic0.rs", "./tests/programs/assignment/arithmetic0.sil");
    }

    #[test]
    fn multiple0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/assignment/multiple0.rs", "./tests/programs/assignment/multiple0.sil");
    }

    #[test]
    fn named0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/assignment/named0.rs", "./tests/programs/assignment/named0.sil");
    }
}

mod compound {
    use super::*;

    #[test]
    fn nested0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/compound/nested0.rs", "./tests/programs/compound/nested0.sil");
    }
}

mod functions {
    use super::*;

    #[test]
    fn call0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/functions/call0.rs", "./tests/programs/functions/call0.sil");
    }
}

mod literals {
    use super::*;

    #[test]
    fn int0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/literals/int0.rs", "./tests/programs/literals/int0.sil");
    }

    #[test]
    fn float0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/literals/float0.rs", "./tests/programs/literals/float0.sil");
    }

    #[test]
    fn str0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/literals/str0.rs", "./tests/programs/literals/str0.sil");
    }

    #[test]
    fn null0() {
        let args = &mut vec![];
        default_args(args);
        run_test(args, "./tests/programs/literals/null0.rs", "./tests/programs/literals/null0.sil");
    }
}
