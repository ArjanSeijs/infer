(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *)

open IStd
open RustMir2TextualTest

let%expect_test "after_lifetime" =
  test "./general/after_lifetime.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define after_lifetime::main() : void {
        local var_0: void, var_1: *int, var_2: void, var_3: int, var_4: *int, var_5: *int, var_6: int
        #node_0:
            store &var_3 <- 50:int
            store &var_5 <- &var_3:*int
            n0:*int = load &var_5
            store &var_4 <- n0:*int
            n1:*int = load &var_4
            store &var_1 <- n1:*int
            store &var_2 <- null:void
            n2:*int = load &var_1
            n3:int  = load n2
            store &var_6 <- n3:int
            store &var_0 <- null:void
            ret var_0

    }

    |}]


let%expect_test "null_latent" =
  test "./general/null_latent.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define null_latent::latent(i: int) : void {
        local var_0: void, var_1: int, var_2: int, var_3: *int, var_4: void, var_5: int, var_6: int, var_7: *int, var_8: *int, var_9: *int

        #node_0:
            store &var_2 <- 10
            n0 = load &i
            store &var_6 <- n0
            n1 = __sil_gt(var_6, 0)
            store &var_5 <- n1
            if var_5 then jmp node_1 else jmp node_2

        #node_1:
            store &var_7 <- null
            n2:*int = load &var_7
            store &var_3 <- n2:*int
            store &var_4 <- null
            jmp node_4

        #node_2:
            store &var_9 <- &var_2
            n3:*int = load &var_9
            store &var_3 <- n3:*int
            store &var_4 <- null
            jmp node_4

        #node_4:
            n4:*int = load &var_3
            n5:int  = load n4
            store &var_1 <- n5
            store &var_0 <- null
            ret var_0

    }

    define null_latent::main() : void {
        local var_0: void

        #node_0:
            n0 = null_latent::latent(-1)
            store &var_0 <- null
            ret var_0

    }

    |}]


let%expect_test "null" =
  test "./general/null.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define null::main() : void {
    local var_0: void, var_1: *int, var_2: int
    #node_0:
        store &var_1 <- null
        jmp node_1

    #node_1:
        n0:*int = load &var_1         
        n1:int  = load n0         
        store &var_2 <- n1
        store &var_0 <- null
        ret var_0

    }

    |}]


let%expect_test "out_of_bounds" =
  test "./general/out_of_bounds.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define out_of_bounds::latent(i: int) : void {
    local var_0: void, var_1: int, var_2: int, var_3: *int, var_4: void, var_5: int, var_6: int, var_7: *int, var_8: *int, var_9: *int

        #node_0:
            store &var_2 <- 10
            n0 = load &i
            store &var_6 <- n0
            n1 = __sil_gt(var_6, 0)
            store &var_5 <- n1
            if var_5 then jmp node_0 else jmp node_2

        #node_1:
            store &var_7 <- null
            n2:*int = load &var_7
            store &var_3 <- n2
            store &var_4 <- null
            jmp node_3

        #node_2:
            store &var_9 <- &var_2
            n3:*int = load &var_9
            store &var_3 <- n3
            store &var_4 <- null
            jmp node_3

        #node_3:
            n4:*int = load &var_3
            n5:int  = load n4
            store &var_1 <- n5
            store &var_0 <- null
            ret var_0
    }

    define out_of_bounds::main() : void {
        local var_0: void

        #node_0:
            n0 = out_of_bounds::latent(-1)
            store &var_0 <- null
            ret var_0
    }

    |}]
