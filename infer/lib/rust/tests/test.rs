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

mod operands {
    use super::*;

    mod const {
        use super::*;
        
        #[test]
        fn literal_float() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/const/literal_float.rs", "./tests/programs/operands/const/literal_float.sil");
        }

        #[test]
        fn literal_int() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/const/literal_int.rs", "./tests/programs/operands/const/literal_int.sil");
        }

        #[test]
        fn literal_mixed() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/const/literal_mixed.rs", "./tests/programs/operands/const/literal_mixed.sil");
        }

        #[test]
        fn literal_str() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/const/literal_str.rs", "./tests/programs/operands/const/literal_str.sil");
        }
        
        #[test]
        fn literal_unit() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/const/literal_unit.rs", "./tests/programs/operands/const/literal_unit.sil");
        }    
    }

    mod r#move {
        use super::*;

        #[test]
        fn basic_move() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/move/basic_move.rs", "./tests/programs/operands/move/basic_move.sil");
        }

        #[test]
        fn move_chain() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/move/move_chain.rs", "./tests/programs/operands/move/move_chain.sil");
        }
        
        #[test]
        fn move_from_field() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/move/move_from_field.rs", "./tests/programs/operands/move/move_from_field.sil");
        }

        #[test]
        fn move_from_return() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/move/move_from_return.rs", "./tests/programs/operands/move/move_from_return.sil");
        }
        
        #[test]
        fn move_from_tuple() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/move/move_from_tuple.rs", "./tests/programs/operands/move/move_from_tuple.sil");
        }

        #[test]
        fn struct_move() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/move/struct_move.rs", "./tests/programs/operands/move/struct_move.sil");
        }
    }

    mod copy {
        use super::*;

        #[test]
        fn basic_copy() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/copy/basic_copy.rs", "./tests/programs/operands/copy/basic_copy.sil");
        }
        
        #[test]
        fn copy_from_return() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/copy/copy_from_return.rs", "./tests/programs/operands/copy/copy_from_return.sil");
        }

        #[test]
        fn copy_in_exp() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/copy/copy_in_exp.rs", "./tests/programs/operands/copy/copy_in_exp.sil");
        }
        
        #[test]
        fn nested_copy() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/copy/nested_copy.rs", "./tests/programs/operands/copy/nested_copy.sil");
        }
        
        #[test]
        fn struct_copy() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/operands/copy/struct_copy.rs", "./tests/programs/operands/copy/struct_copy.sil");
        }
    }
}

mod rvalues {
    use super::*;

    mod binop {
        use super::*;

        mod add {
            use super::*;

            #[test]
            fn add_i16() {
                let args = &mut vec![];
                default_args(args);
                run_test(args, "./tests/programs/rvalues/binop/add/add_i16.rs", "./tests/programs/rvalues/binop/add/add_i16.sil");
            }

            #[test]
            fn add_i32() {
                let args = &mut vec![];
                default_args(args);
                run_test(args, "./tests/programs/rvalues/binop/add/add_i32.rs", "./tests/programs/rvalues/binop/add/add_i32.sil");
            }

            #[test]
            fn add0() {
                let args = &mut vec![];
                default_args(args);
                run_test(args, "./tests/programs/rvalues/binop/add/add0.rs", "./tests/programs/rvalues/binop/add/add0.sil");
            }
        }

        mod sub {
            use super::*;

            #[test]
            fn sub_u128() {
                let args = &mut vec![];
                default_args(args);
                run_test(args, "./tests/programs/rvalues/binop/sub/sub_u128.rs", "./tests/programs/rvalues/binop/sub/sub_u128.sil");
            }
        }
    }

    mod mut_raw_ptr {
        use super::*;

        #[test]
        fn mut_raw_ptr() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/rvalues/mut_raw_ptr/mut_raw_ptr0.rs", "./tests/programs/rvalues/mut_raw_ptr/mut_raw_ptr0.sil");
        }
    }

    mod mut_ref {
        use super::*;

        #[test]
        fn mut_ref() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/rvalues/mut_ref/mut_ref0.rs", "./tests/programs/rvalues/mut_ref/mut_ref0.sil");
        }
    }

    mod raw_ptr {
        use super::*;

        #[test]
        fn raw_ptr() {
            let args = &mut vec![]; 
            default_args(args);
            run_test(args, "./tests/programs/rvalues/raw_ptr/raw_ptr0.rs", "./tests/programs/rvalues/raw_ptr/raw_ptr0.sil");
        }
    }

    mod r#ref {
        use super::*;

        #[test]
        fn ref0() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/rvalues/ref/ref0.rs", "./tests/programs/rvalues/ref/ref0.sil");
        }
    }

    mod unop {
        use super::*;

        #[test]
        fn unop0() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/rvalues/unop/unop0.rs", "./tests/programs/rvalues/unop/unop0.sil");
        }
    }
}

mod statements {
    use super::*;

    mod assign {
        use super::*;
        
        #[test]
        fn assign_binop() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/assign/assign_binop.rs", "./tests/programs/statements/assign/assign_binop.sil");
        }

        #[test]
        fn assign_cast() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/assign/assign_cast.rs", "./tests/programs/statements/assign/assign_cast.sil");
        }
        
        #[test]
        fn assign_mut_ref() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/assign/assign_mut_ref.rs", "./tests/programs/statements/assign/assign_mut_ref.sil");
        }
        
        #[test]
        fn assign_ref() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/assign/assign_ref.rs", "./tests/programs/statements/assign/assign_ref.sil");
        }
        
        #[test]
        fn assign_tuple() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/assign/assign_tuple.rs", "./tests/programs/statements/assign/assign_tuple.sil");
        }
    }

    mod storage_live {
        use super::*;

        #[test]
        fn storage_live() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/storage_live/storage_live.rs", "./tests/programs/statements/storage_live/storage_live.sil");
        }
    }

    mod storage_dead {
        use super::*;

        #[test]
        fn storage_dead_shadow() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/statements/storage_dead/storage_dead_shadow.rs", "./tests/programs/statements/storage_dead/storage_dead_shadow.sil");
        }
    }
}

mod terminator {
    use super::*;
    
    mod call {
        use super::*;

        #[test]
        fn call() {
            let args = &mut vec![];
            default_args(args);
            run_test(args, "./tests/programs/terminator/call/call0.rs", "./tests/programs/terminator/call/call0.sil");
        }
    }
}
