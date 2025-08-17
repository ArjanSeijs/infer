(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *)

open IStd
open RustMir2TextualTest

(* Tests for const *)
let%expect_test "literal_float" =
  test "./operands/const/literal_float.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define literal_float::main() : void {
      local var_0: void, var_1: float
      #node_0:
          store &var_1 <- 3.14
          store &var_0 <- null
          ret var_0

    }

    |}]


let%expect_test "literal_int" =
  test "./operands/const/literal_int.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define literal_int::main() : void {
      local var_0: void, var_1: int
      #node_0:
          store &var_1 <- 42
          store &var_0 <- null
          ret var_0
          
    }

    |}]


let%expect_test "literal_mixed" =
  test "./operands/const/literal_mixed.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define literal_mixed::main() : void {
      local var_0: void, var_1: int, var_2: float, var_3: *String
      #node_0:
          store &var_1 <- 42
          store &var_2 <- 3.14
          store &var_3 <- "hi"
          store &var_0 <- null
          ret var_0

    }

    |}]


let%expect_test "literal_str" =
  test "./operands/const/literal_str.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define literal_str::main() : void {
      local var_0: void, var_1: *String
      #node_0:
          store &var_1 <- "hello"
          store &var_0 <- null
          ret var_0
        
    }

    |}]


let%expect_test "literal_unit" =
  test "./operands/const/literal_str.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define literal_unit::main() : void {
      local var_0: void
      #node_0:
          store &var_0 <- null
          ret var_0
        
    }

    |}]


(* Tests for copy *)
let%expect_test "basic_copy" =
  test "./operands/copy/basic_copy.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define basic_copy::main() : void {
      local var_0: void, var_1: int, var_2: int
      #node_0:
          store &var_1 <- 42
          store &var_2 <- var_1
          store &var_0 <- null
          ret var_0
        
    }

    |}]


let%expect_test "copy_from_return" =
  test "./operands/copy/copy_from_return.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define copy_from_return::make() : int {
      local var_0: int
      #node_0:
          store &var_0 <- 5
          ret var_0

    }

    define copy_from_return::main() : void {
      local var_0: void, var_1: int, var_2: int
      #node_0:
          store &var_0 <- copy_from_return::make()
          jmp node_1

      #node_1:
          store &var_1 <- var_0
          store &var_0 <- null
          ret var_0

    }

    |}]


let%expect_test "copy_in_exp" =
  test "./operands/copy/copy_in_exp.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define copy_in_exp::main() : void {
      local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int, var_5: int
      #node_0:
          store &var_1 <- 10
          store &var_2 <- 20
          store &var_4 <- var_0
          store &var_5 <- var_1
          store &var_3 <- __sil_plusa_int(var_4, var_5)
          store &var_0 <- null
          ret var_0

    }

    |}]


let%expect_test "nested_copy" =
  test "./operands/copy/nested_copy.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define nested_copy::main() : void {
      local var_0: void, var_1: int, var_2: int
      #node_0:
          store &var_1 <- 100
          store &var_2 <- var_1
          store &var_0 <- null
          ret var_0

    }

    |}]


let%expect_test "struct_copy" =
  test "./operands/copy/struct_copy.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    type Point = { x: int; y: int }

    define struct_copy::main() : void {
      local var_0: void, var_1: Point, var_2: int
      #node_0:
          store &var_1.Point.x <- 1
          store &var_1.Point.y <- 2
          store &var_2 <- &var_1.Point.x
          store &var_0 <- null
          ret var_0

    }

    |}]


(* Tests for move *)
let%expect_test "basic_move" =
  test "./operands/move/basic_move.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define basic_move::main() : void {
      local var_0: void, var_1: *String, var_2: *String
      #node_0:
          store &var_1 <- "hello"
          store &var_2 <- var_1
          store &var_0 <- null
          ret var_0

    }

    |}]


let%expect_test "move_chain" =
  test "./operands/move/move_chain.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define move_chain::main() : void {
      local var_0: void, var_1: *String, var_2: *String, var_3: *String
      #node_0:
          store &var_1 <- "hello"
          store &var_2 <- var_1
          store &var_3 <- var_2
          n0 = load &var_3
          n1 = __sil_free(n0)
          store &var_0 <- null
          ret var_0
        
    }

    |}]


let%expect_test "move_from_field" =
  test "./operands/move/move_from_field.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    type Wrapper = { val: *String }

    define move_from_field::main() : void {
      local var_0: void, var_1: Wrapper, var_2: *String, var_3: *String
      #node_0:
          n0 = "hi"
          store &var_2 <- n0
          store &var_1.Wrapper.val <- var_2
          n2 = var_1.Wrapper.val
          store &var_3 <- n2
          n3 = load &var_3
          n4 = __sil_free(n3)
          store &var_0 <- null
          ret var_0
        
    }
    
    |}]


let%expect_test "move_from_return" =
  test "./operands/move/move_from_return.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define move_from_return::make_string() : *String {
      local var_0: *String
      #node_0:
          n0 = "hello"
          store &var_0 <- n0
          ret var_0

    }

    define move_from_return::main() : void {
      local var_0: void, var_1: *String
      #node_0:
          n0 = move_from_return::make_string()
          store &var_1 <- n0

          n1 = load &var_1
          n2 = __sil_free(n1)

          store &var_0 <- null

          ret var_0

    }

    |}]


let%expect_test "move_from_tuple" =
  test "./operands/move/move_from_tuple.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    type tuple_string_int = { f0: *String; f1: int }

    define move_from_tuple::main() : void {
      local var_0: void, var_1: tuple_string_int, var_2: *String, var_3: *String
      #node_0:
          store &var_2 <- "hello"
          store &var_1.tuple_string_int.f0 <- var_2
          store &var_1.tuple_string_int.f1 <- 42

          n2 = var_1.tuple_string_int.f0
          store &var_3 <- n2

          n3 = load &var_3
          n4 = __sil_free(n3)

          store &var_0 <- null

          ret var_0

    }

    |}]


let%expect_test "struct_move" =
  test "./operands/move/struct_move.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    type Point = { x: int; y: int }

    define struct_move::main() : void {
      local var_0: void, var_1: Point, var_2: Point
      #node_0:
          n0 = &var_1
          store &var_1.Point.x <- 1
          store &var_1.Point.y <- 2
          store &var_2 <- var_1
          store &var_0 <- null
          ret var_0

    }

    |}]
