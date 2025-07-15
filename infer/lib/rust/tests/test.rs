#![feature(rustc_private)]
use infer_rustc_mir::call_compiler;

fn run_test(args: &mut Vec<String>, file_name: &str) {
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

    // Finally push filename
    args.push(file_name.to_string());
    call_compiler(&args);
}

#[allow(unused)]
fn print_mir(args: &mut Vec<String>, file_name: &str) {
    args.push("-Z".to_string());
    args.push("unpretty=mir".to_string());
    run_test(args, file_name);
}

#[test]
fn add0() {
    run_test(&mut vec![], "./tests/example_files/add0.rs");
    // print_mir(&mut vec![], "./tests/example_files/add0.rs");
}
