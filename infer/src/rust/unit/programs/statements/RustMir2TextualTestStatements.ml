(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *)

open IStd
open RustMir2TextualTest

(* Tests for assign statement *)
let%expect_test "assign_binop" =
  test "./statements/assign/assign_binop.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define assign_binop::main() : void {
    local var_0: void, var_1: int
    #node_0:
        store &var_1 <- __sil_plusa_int(2,3)
        store &var_0 <- null
        ret var_0
        
    }

  |}]


let%expect_test "assign_cast" =
  test "./statements/assign/assign_cast.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define assign_cast::main() : void {
    local var_0: void, var_1: long, var_2: int
    #node_0:
        n0:long = load &var_1
        n1 = __sil_cast(<int>, n0)
        store &var_2 <- n1:int

        store &var_0 <- null:void
        ret var_0
        
    }

  |}]


let%expect_test "assign_tuple" =
  test "./statements/assign/assign_tuple.ullbc" ;
  [%expect
    {|
   .source_language = "Rust"

    type tuple_int_int = { f0: int; f1: int }

    define assign_tuple::main() : void {
    local var_0: void, var_1: tuple_int_int
    #node_0:
        store &var_1.tuple_int_int.f0 <- 1
        store &var_1.tuple_int_int.f1 <- 2
        store &var_0 <- null:void
        ret var_0
        
    }

  |}]


(* Tests for storage dead *)
let%expect_test "storage_dead_shadow" =
  test "./statements/storage_dead/storage_dead_shadow.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define storeage_dead_shadow::main() : void {
    local var_0: void, var_1: int, var_2: void, var_3: int
    #node_0:
        store &var_1 <- 1
        store &var_3 <- 2
        store &var_2 <- null
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for storage live *)
let%expect_test "storage_live0" =
  test "./statements/storage_live/storage_live0.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define storage_live0::main() : void {
    local var_0:void, var_1: int
    #node_0:
        store &var_1 <- 1
        store &var_0 <- null
        ret var_0
        
    }

  |}]
