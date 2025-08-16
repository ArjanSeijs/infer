(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *)

open RustMir2TextualTest

(* Tests for call terminator *)
let%expect_test "basic_call" =
  test "./terminators/call/basic_call.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define basic_call::main() : void {
    local var_0: void
    #node_0:
        store &var_0 <- bar()
        jmp node_1

    #node_1:
        ret var_0
    }

    define bar() : void {
    local var_0: void
    #node_0:
        store &var_0 <- null
        ret var_0
    }

  |}]


let%expect_test "call_res_ignored" =
  test "./terminators/call/call_res_ignored.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define call_res_ignored::main() : void {
    local var_0: void, var_1: int, var_2: int

    #node_0:
        store &var_1 <- 0
        n0 = load &var_1
        store &var_2 <- n0
        n1 = load & var_2
        n2 = call_res_ignored::log_value(n1)
        n3 = __sil_plusa_int(n1, 1)
        store &var_0 <- null
        ret var_0
        
    }

    define call_res_ignored::log_value(v: int) : void {
    local var_0: void

    #node_0:
        store &var_0 <- null
        ret var_0
        
    }

  |}]


let%expect_test "call_with_args" =
  test "./terminators/call/call_with_args.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define call_with_args::call_with_args(x: int, y: int) : int {
    local var_3: int, var_4: int

    #node_0:
        n0 = load &x
        store &var_3 <- n0
        n1 = load &y
        store &var_4 <- n1
        n2 = call_with_args::add(var_3, var_4)
        ret n2
        
    }

    define call_with_args::add(a: int, b: int) : int {
    local var_3: int, var_4: int

    #node_0:
        n0 = load &a
        store &var_3 <- n0
        n1 = load &b
        store &var_4 <- n1
        n2 = __sil_plusa_int(var_3, var_4)
        ret n2

    }

    define call_with_args::main() : void {
    local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int

    #node_0:
        store &var_1 <- 0
        store &var_2 <- 1
        n0 = load &var_1
        store &var_3 <- n0
        n1 = load &var_2
        store &var_4 <- n1
        n2 = call_with_args::call_with_args(var_3, var_4)
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for drop terminator *)
let%expect_test "drop_break" =
  test "./terminators/drop/drop_break.ullbc" ;
  [%expect
    {|
   .source_language = "Rust"

    define drop_break::drop_term_break_local(n: int) : int {
    local var_0: void, var_1: void, var_2: *String, var_3: void, var_4: bool, var_5: int, var_6: void, var_7: void

    #node_0:
        jmp node_1

    #node_1:
        n0 = "hi"
        store &var_2 <- n0
        jmp node_2

    #node_2:
        n1 = load &n
        n2 = __sil_gt(n1, 0)
        if n2 then jmp node_3 else jmp node_4

    #node_3:
        n3 = load &var_2
        store &var_6 <- n3
        n4 = __sil_free(var_6)
        jmp node_6

    #node_4:
        n5 = load &n
        n6 = __sil_minusa_int(n5, 1)
        store &n <- n6
        n7 = load &var_2
        store &var_7 <- n7
        n8 = __sil_free(var_7)
        jmp node_5

    #node_5:
        jmp node_1

    #node_6:
        ret n
    }

    define drop_break::main() : void {
    local var_0: void, var_1: int, var_2: int

    #node_0:
        store &var_1 <- 0
        n0 = load &var_1
        store &var_2 <- n0
        n1 = drop_break::drop_term_break_local(var_2)
        store &var_0 <- null
        ret var_0

    }

  |}]


let%expect_test "drop_continue" =
  test "./terminators/drop/drop_continue.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define drop_continue::drop_term_continue_local(n: int) : int {
    local var_0: void, var_1: void, var_2: *String, var_3: void, var_4: bool, var_5: int, var_6: void, var_7: void, var_8: void

    #node_0:
        jmp node_1

    #node_1:
        n0 = "x"
        store &var_2 <- n0
        jmp node_2

    #node_2:
        n1 = load &n
        n2 = __sil_mod(n1, 2)
        n3 = __sil_eq(n2, 0)
        if n3 then jmp node_3 else jmp node_4

    #node_3:
        n4 = load &n
        n5 = __sil_plusa_int(n4, 1)
        store &n <- n5
        n6 = load &var_2
        store &var_6 <- n6
        n7 = __sil_free(var_6)
        jmp node_9

    #node_4:
        n8 = load &n
        n9 = __sil_plusa_int(n8, 1)
        store &n <- n9
        n10 = load &n
        n11 = __sil_gt(n10, 10)
        if n11 then jmp node_5 else jmp node_6

    #node_5:
        n12 = load &var_2
        store &var_7 <- n12
        n13 = __sil_free(var_7)
        jmp node_8

    #node_6:
        n12 = load &var_2
        store &var_8 <- n12
        n13 = __sil_free(var_8)
        jmp node_7

    #node_7:
        jmp node_1

    #node_8:
        ret n

    #node_9:
        jmp node_1
    }

    define drop_continue::main() : void {
    local var_0: void, var_1: int, var_2: int

    #node_0:
        store &var_1 <- 0
        n0 = load &var_1
        store &var_2 <- n0
        n1 = drop_continue::drop_term_continue_local(var_2)
        store &var_0 <- null
        ret var_0
    }

  |}]


let%expect_test "drop_return_early" =
  test "./terminators/drop/drop_return_early.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define drop_return_early(flag: bool) : int {
        local var_2: *String, var_3: *String, var_4: void, var_5: void

        #node_0:
            n0 = "A"
            store &var_2 <- n0
            n1 = "B"
            store &var_3 <- n1
            jmp node_1

        #node_1:
            n2 = load &flag
            if n2 then jmp node_2 else jmp node_4

        #node_2:
            n3 = load &var_3
            store &var_4 <- n3
            n4 = __sil_free(var_4)
            jmp node_3

        #node_3:
            n5 = load &var_2
            store &var_5 <- n5
            n6 = __sil_free(var_5)
            jmp node_6

        #node_4:
            n7 = load &var_2
            store &var_4 <- n7
            n8 = __sil_free(var_4)
            n9 = load &var_3
            store &var_5 <- n9
            n10 = __sil_free(var_5)
            jmp node_5

        #node_5:
            ret 0

        #node_6:
            ret 1
            
    }

    define drop_return_early::main() : void {
    local var_0: void, var_1: bool, var_2: bool

    #node_0:
        store &var_1 <- false
        n0 = load &var_1
        store &var_2 <- n0
        n1 = drop_return_early(var_2)
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for goto *)
let%expect_test "basic_loop" =
  test "./terminators/goto/basic_loop.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define basic_loop::goto_loop(n: int) : int {
    #node_0:
        jmp node_1

    #node_1:
        n0 = load &n
        n1 = __sil_plusa_int(n0, 1)
        store &n <- n1
        n2 = __sil_gt(n1, 5)
        if n2 then jmp node_2 else jmp node_3

    #node_2:
        ret n

    #node_3:
        jmp node_1
        
    }

    define basic_loop::main() : void {
    local x: int, var_0: void, var_1: int

    #node_0:
        store &x <- 0
        n0 = load &x
        store &var_1 <- n0
        n1 = basic_loop::goto_loop(var_1)
        store &var_0 <- null
        ret var_0

    }

  |}]


let%expect_test "break_outer" =
  test "./terminators/goto/break_outer.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define break_outer::goto_with_break_outer(a: int, b: int) : int {
    #node_0:
        jmp node_1

    #node_1:
        n0 = load &a
        n1 = load &b
        n2 = __sil_gt(n0, n1)
        if n2 then jmp node_2 else jmp node_3

    #node_2:
        n3 = load &a
        n4 = load &b
        n5 = __sil_plusa_int(n3, n4)
        ret n5

    #node_3:
        n6 = load &b
        n7 = __sil_plusa_int(n6, 1)
        store &b <- n7
        jmp node_1
        
    }

    define break_outer::main() : void {
    local x: int, y: int, var_0: void, var_1: int, var_2: int, var_3: int

    #node_0:
        store &x <- 0
        store &y <- 1
        n0 = load &x
        store &var_1 <- n0
        n1 = load &y
        store &var_2 <- n1
        n2 = break_outer::goto_with_break_outer(var_1, var_2)
        store &var_0 <- null
        ret var_0

    }

  |}]


let%expect_test "break_with_continue" =
  test "./terminators/goto/break_with_continue.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define loop_with_continue::goto_with_continue(x: int, y: int) : int {
    #node_0:
        jmp node_1

    #node_1:
        n0 = load &x
        n1 = load &y
        n2 = __sil_lt(n0, n1)
        if n2 then jmp node_2 else jmp node_3

    #node_2:
        n3 = load &x
        n4 = __sil_plusa_int(n3, 1)
        store &x <- n4
        jmp node_1

    #node_3:
        ret x
        
    }

    define loop_with_continue::main() : void {
    local x: int, y: int, var_0: void, var_1: int, var_2: int

    #node_0:
        store &x <- 0
        store &y <- 1
        n0 = load &x
        store &var_1 <- n0
        n1 = load &y
        store &var_2 <- n1
        n2 = loop_with_continue::goto_with_continue(var_1, var_2)
        store &var_0 <- null
        ret var_0

    }

  |}]


let%expect_test "int_comparison" =
  test "./terminators/switch_int/int_comparison.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define int_comparison::swi_cmp(x: int, y: int) : int {
    local var_3: bool, var_4: int, var_5: int
    #node_0:
        n0 = load &x
        store &var_4 <- n0
        n1 = load &y
        store &var_5 <- n1
        n2 = __sil_gt(var_4, var_5)
        if n2 then jmp node_1 else jmp node_2

    #node_1:
        ret var_4

    #node_2:
        ret var_5

    }

    define int_comparison::main() : void {
    local x: int, y: int, var_0: void, var_1: int, var_2: int
    #node_0:
        store &x <- 0
        store &y <- 1
        n0 = load &x
        store &var_1 <- n0
        n1 = load &y
        store &var_2 <- n1
        n2 = int_comparison::swi_cmp(var_1, var_2)
        store &var_0 <- null
        ret var_0
    }

  |}]


let%expect_test "negated_bool" =
  test "./terminators/switch_int/negated_bool.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define negated_bool::swi_not(b: bool) : int {
    local var_2: bool
    #node_0:
        n0 = load &b
        store &var_2 <- n0
        if var_2 then jmp node_2 else jmp node_1

    #node_1:
        ret 10

    #node_2:
        ret 20

    }

    define negated_bool::main() : void {
    local x: bool, var_0: void, var_1:bool

    #node_0:
        store &x <- false
        n0 = load &x
        store &var_1 <- n0
        n1 = negated_bool::swi_not(var_1)
        store &var_0 <- null
        ret var_0

    }

  |}]


let%expect_test "nested" =
  test "./terminators/switch_int/nested.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define nested::swi_nested(a: int, b: int) : int {
    local var_3: bool, var_4: int, var_5: bool, var_6: int

    #node_0:
        n0 = load &a
        store &var_4 <- n0
        n1 = __sil_ge(var_4, 0)
        if n1 then jmp node_1 else jmp node_4

    #node_1:
        n2 = load &b
        store &var_6 <- n2
        n3 = __sil_eq(var_6, 0)
        if n3 then jmp node_2 else jmp node_3

    #node_2:
        ret 1

    #node_3:
        ret 2

    #node_4:
        ret 3

    }

    define nested::main() : void {
    local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int

    #node_0:
        store &var_1 <- 0
        store &var_2 <- 1
        n0 = load &var_1
        store &var_3 <- n0
        n1 = load &var_2
        store &var_4 <- n1
        n2 = nested::swi_nested(var_3, var_4)
        store &var_0 <- null
        ret var_0

    }

  |}]


let%expect_test "parity_check" =
  test "./terminators/switch_int/parity_check.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define parity_check::swi_parity(x: int) : int {
        local var_3: int, var_4: int

        #node_0:
            n0:int = load &x
            store &var_4 <- n0:int

            n1:int = load &var_4
            n2 = __sil_mod(n1, 2)
            store &var_3 <- n2

            n3:int = load &var_3
            n4 = __sil_eq(n3, 0)
            if n4 then jmp node_1 else jmp node_2

        #node_1:
            n5:int = load &x
            ret n5

        #node_2:
            n6:int = load &x
            n7 = __sil_neg(n6)
            ret n7
    }

    define parity_check::main() : void {
        local var_0: void, var_1: int
        #node_0:
            store &var_1 <- 0
            n0:int = load &var_1
            n1 = parity_check::swi_parity(n0)
            store &var_0 <- null
            ret var_0
    }

  |}]
