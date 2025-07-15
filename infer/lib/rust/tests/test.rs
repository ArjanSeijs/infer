use infer_rustc_mir::call_compiler; 

fn run_test(args: &mut Vec<String>, file_name: &str) {
    // Do no optimisation on test
    args.push("-Z".to_string());
    args.push("mir-opt-level=0".to_string());
    // Do not require main function
    args.push("--crate-type=lib".to_string());
    // Finally push filename
    args.push(file_name.to_string());
    call_compiler(args);
}

fn print_mir(args: &mut Vec<String>, file_name: &str) {
    args.push("-Z".to_string());
    args.push("unpretty=mir".to_string());
    run_test(args, file_name);
}
