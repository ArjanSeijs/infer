(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *)

open IStd
open RustMir2TextualTest

(* Tests for binop *)
let%expect_test "arithmetic" =
  test "./rvalues/binop/arithmetic.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define arithmetic::plus_minus_mult_basic() : void {
        local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int, var_5: int, var_6: int, var_7: int, var_8: int
        #node_0:
            store &var_1 <- -7
            store &var_2 <- 3
            n0:int = load &var_1
            n1:int = load &var_2
            n2 = __sil_plusa_int(n0, n1)
            n3 = __sil_minusa_int(n0, n1)
            n4 = __sil_mult_int(n0, n1)
            n5 = __sil_eq(n2, -4)
            n6 = __sil_eq(n3, -10)
            n7 = __sil_eq(n4, -21)

            store &var_3 <- 2
            store &var_4 <- 5
            n8:int = load &var_3
            n9:int = load &var_4
            n10 = __sil_plusa_uint(n8, n9)
            n11 = __sil_minusa_uint(n9, n8)
            n12 = __sil_mult_uint(n8, n9)
            n13 = __sil_eq(n10, 7)
            n14 = __sil_eq(n11, 3)
            n15 = __sil_eq(n12, 10)

            store &var_5 <- 1
            store &var_6 <- 0
            n16:int = load &var_5
            n17:int = load &var_6
            n18 = __sil_plusa_uchar(n16, n17)
            n19 = __sil_mult_uchar(n16, n16)
            n20 = __sil_eq(n18, 1)
            n21 = __sil_eq(n19, 1)

            store &var_7 <- 65
            store &var_8 <- 1
            n22:int = load &var_7
            n23:int = load &var_8
            n24 = __sil_plusa_uint(n22, n23)
            n25 = __sil_eq(n24, 66)
            store &var_0 <- null
            ret var_0
            
    }

    define arithmetic::arithmetic_edges_i128_u128() : void {
        local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int
        #node_0:
            store &var_0 <- 340282366920938463463374607431768211455
            store &var_1 <- 1
            n0:int = load &var_0
            n1:int = load &var_1
            n2 = __sil_plusa_u128(n0, n1)
            n3 = __sil_eq(n2, 0)

            store &var_2 <- -170141183460469231731687303715884105728
            store &var_3 <- 1
            n4:int = load &var_2
            n5:int = load &var_3
            n6 = __sil_minusa_128(n4, n5)
            n7 = __sil_eq(n6, 170141183460469231731687303715884105727)
            store &var_0 <- null
            ret var_0

    }

    define arithmetic::main() : void {
        local var_0: void
        #node_0:
            n0 = arithmetic::plus_minus_mult_basic()
            n1 = arithmetic::arithmetic_edges_i128_u128()
            store &var_0 <- null
            ret var_0

    }

  |}]


let%expect_test "bitwise" =
  test "./rvalues/binop/bitwise.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define bitwise::bitwise_ops() : void {
        local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int, var_5: int, var_6: int
        #node_0:
            store &var_1 <- 10
            store &var_2 <- 12
            n0:int = load &var_1
            n1:int = load &var_2
            n2 = __sil_band(n0, n1)
            n3 = __sil_eq(n2, 8)

            store &var_3 <- 10
            store &var_4 <- 5
            n4 = load &var_3
            n5 = load &var_4
            n6 = __sil_bor(n4, n5)
            n7 = __sil_eq(n6, 15)

            store &var_5 <- 15
            n8 = load &var_5
            n9 = load &var_4
            n10 = __sil_bxor(n8, n9)
            n11 = __sil_eq(n10, 10)

            store &var_6 <- -1
            n12:int = load &var_6
            n13:int = load &var_5
            n14 = __sil_band(n12, n13)
            n15 = __sil_eq(n14, 15)
            store &var_0 <- null
            ret var_0

    }

    define bitwise::main() : void {
        local var_0: void
        #node_0:
            n0 = bitwise::bitwise_ops()
            store &var_0 <- null
            ret var_0

    }

  |}]


let%expect_test "comparisons" =
  test "./rvalues/binop/comparisons.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define comparisons::comparisons_ints_and_128() : void {
        local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int, var_5: int, var_6: int, var_7: int, var_8: int, var_9: int, var_10: int
        #node_0:
            store &var_1 <- 3
            store &var_2 <- 5
            n0:int = load &var_0
            n1:int = load &var_1
            n2 = __sil_lt(n0, n1)

            store &var_3 <- 5
            store &var_4 <- 5
            n3:int = load &var_3
            n4:int = load &var_4
            n5 = __sil_ge(n3, n4)

            store &var_5 <- 170141183460469231731687303715884105727
            store &var_6 <- 0
            n6:int = load &var_5
            n7:int = load &var_6
            n8 = __sil_gt(n6, n7)

            store &var_7 <- 340282366920938463463374607431768211455
            store &var_8 <- 0
            n9:int  = load &var_7
            n10:int = load &var_8
            n11 = __sil_gt(n9, n10)

            store &var_9 <- 10
            store &var_10 <- -10
            n12:int = load &var_9
            n13:int = load &var_10
            n14 = __sil_ne(n12, n13)
            store &var_0 <- null
            ret var_0

    }

    define comparisons::comparisons_floats_with_nan() : void {
        local var_0: void, var_1: float, var_2: float
        #node_0:
            store &var_1 <- 0.0:float
            n0:float = load &var_1
            n1 = __sil_divf(n0, n0)

            store &var_2 <- 1.0:float
            n2:float = load &var_2

            n3 = __sil_lt(n1, n2)
            n4 = __sil_lnot(n3)

            n5 = __sil_gt(n1, n2)
            n6 = __sil_lnot(n5)

            n7 = __sil_eq(n1, n1)
            n8 = __sil_lnot(n7)

            n9 = __sil_ne(n1, n1)
            store &var_0 <- null
            ret var_0

    }

    define comparisons::main() : void {
        local var_0: void
        #node_0:
            n0 = comparisons::comparisons_ints_and_128()
            n1 = comparisons::comparisons_floats_with_nan()
            store &var_0 <- null
            ret var_0

    }

  |}]


let%expect_test "div_modulo" =
  test "./rvalues/binop/div_modulo.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define div_modulo::divi_and_mod_signed_unsigned() : void {
        local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int, var_5: int, var_6: int
        #node_0:
            store &var_1 <- 7
            store &var_2 <- 3
            n0:int = load &var_1
            n1:int = load &var_2
            n2 = __sil_divi(n0, n1)
            n3 = __sil_eq(n2, 2)

            n4 = __sil_mod(n0, n1)
            n5 = __sil_eq(n4, 1)

            store &var_3 <- 7
            store &var_4 <- -3
            n6:int = load &var_3
            n7:int = load &var_4
            n8 = __sil_divi(n6, n7)
            n9 = __sil_eq(n8, -2)

            n10 = __sil_mod(n6, n7)
            n11 = __sil_eq(n10, 1)

            store &var_5 <- -7
            store &var_6 <- 3
            n12:int = load &var_5
            n13:int = load &var_6
            n14 = __sil_divi(n12, n13)
            n15 = __sil_eq(n14, -2)

            n16 = __sil_mod(n12, n13)
            n17 = __sil_eq(n16, -1)
            store &var_0 <- null
            ret var_0

    }

    define div_modulo::divide_by_zero_panics() : void {
        local var_0: void, var_1: int, var_2: int
        #node_0:
            store &var_1 <- 1
            store &var_2 <- 0
            n2:int = load &var_1
            n3:int = load &var_2
            n4 = __sil_divi(n2, n3)
            store &var_0 <- null
            ret var_0

    }

    define div_modulo::divf_nan_inf() : void {
        local var_1: float, var_2: float
        #node_0:
            store &var_1 <- 0.0:float
            store &var_2 <- 1.0:float
            n0:float = load &var_1
            n1:float = load &var_2
            n2 = __sil_divf(n1, n0)

            n4 = __sil_divf(n1, n0)
            n5 = __sil_eq(n2, n4)
            n6 = __sil_gt(n2, 0.0)
            n7 = __sil_land(n5, n6)

            n8 = __sil_divf(n0, n0)
            n9 = __sil_ne(n8, n8)
            store &var_0 <- null
            ret var_0

    }

    define div_modulo::main() : void {
        local var_0: void
        #node_0:
            n0 = div_modulo::divi_and_mod_signed_unsigned()
            n1 = div_modulo::divide_by_zero_panics()
            n2 = div_modulo::divf_nan_inf()
            store &var_0 <- null
            ret var_0

    }
    s
  |}]


let%expect_test "logical" =
  test "./rvalues/binop/logical.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define logical::side_effect(ptr: *int) : bool {
        #node_0:
            n0:int = load ptr
            n1 = __sil_plusa_int(n0, 1)
            store &ptr <- n1
            ret true

    }

    define logical::logical_and_short_circuit() : void {
        local var_0: void, var_1: int, var_2: bool, var_3: bool, var_4: bool, var_5: bool
        #node_0:
            store &var_1 <- 0

            store &var_2 <- false
            n0 = load &var_2
            n1 = __sil_land(n0, false)
            store &var_3 <- n1
            n2 = __sil_lnot(n1)
            n3 = load &var_1
            n4 = __sil_eq(n3, 0)

            store &var_2 <- true
            n5:bool = load &var_2
            n6 = logical::side_effect(&var_1)
            n7 = __sil_land(n5, n6)
            n8 = __sil_eq(n7, true)  
            n9 = load &var_1      
            n10 = __sil_eq(n9, 1)

            store &var_0 <- null
            ret var_0

    }

    define logical::logical_or_short_circuit() : void {
        local var_0: void, var_1: int, var_2: bool, var_3: bool, var_4: bool, var_5: bool
        #node_0:
            store &var_1 <- 0
            store &var_2 <- true:bool
            n0:bool = load &var_2
            n1 = __sil_lor(n0, false)
            n2 = __sil_eq(n1, true)  
            n3 = load &var_1 
            n4 = __sil_eq(n3, 0)

            store &var_2 <- false:bool
            n5:bool = load &var_2
            n6 = logical::side_effect(&var_1)
            n7 = __sil_lor(n5, n6)
            n8 = __sil_eq(n7, true)
            n9 = load &var_1
            n10 = __sil_eq(n9, 1)

            store &var_0 <- null
            ret var_0

    }

    define logical::main() : void {
        local var_0: void
        #node_0:
            n0 = logical::logical_and_short_circuit()
            n1 = logical::logical_or_short_circuit()
            store &var_0 <- null
            ret var_0

    }

  |}]


let%expect_test "shifts" =
  test "./rvalues/binop/shifts.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define shifts::shifts_signed_vs_unsigned() : void {
        local var_0: void, var_1: int, var_2: int, var_3: int, var_4: int, var_5: int, var_6: int
        #node_0:
            store &var_1 <- 1
            store &var_2 <- 3
            n0:int = load &var_1
            n1:int = load &var_2
            n2 = __sil_shiftlt(n0, n1)
            n3 = __sil_eq(n2, 8)

            store &var_3 <- 128
            store &var_4 <- 7
            n4:int = load &var_3
            n5:int = load &var_4
            n6 = __sil_shiftrt(n4, n5)
            n7 = __sil_eq(n6, 1)

            store &var_5 <- -2
            store &var_6 <- 1
            n8:int = load &var_5
            n9:int = load &var_6
            n10 = __sil_shiftrt(n8, n9)
            n11 = __sil_eq(n10, -1)

            store &var_0 <- null:void
            ret var_0

    }

    define shifts::shift_too_large_panics_in_debug() : void {
        local var_0: void, var_1: int, var_2: int
        #node_0:
            store &var_1 <- 1
            store &var_2 <- 8
            n0:int = load &var_1
            n1:int = load &var_2
            n2 = __sil_shiftlt(n0, n1)

            store &var_0 <- null:void
            ret var_0

    }

    define shifts::main() : void {
        local var_0: void
        #node_0:
            n0 = shifts::shifts_signed_vs_unsigned()
            n1 = shifts::shift_too_large_panics_in_debug()
            store &var_0 <- null
            ret var_0

    }

  |}]


(* Tests for mutable raw pointers *)
let%expect_test "mut_raw_ptr0" =
  test "./rvalues/mut_raw_ptr/mut_raw_ptr0.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define mut_raw_ptr0::main() : void {
    local var_0: void, var_1: int, var_2: *int, var_3: *int
    #node_0:
        store &var_0 <- 10
        store &var_2 <- &var_1
        store &var_1 <- var_3
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for mutable references *)
let%expect_test "mut_ref0" =
  test "./rvalues/mut_ref/mut_ref0.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define mut_ref0::main() : void {
    local var_0: void, var_1: int, var_2: *int

    #node_0:
        store &var_1 <- 10
        store &var_2 <- &var_1
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for raw pointers *)
let%expect_test "raw_ptr0" =
  test "./rvalues/raw_ptr/raw_ptr0.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define raw_ptr0::main() : void {
    local var_0: void, var_1: int, var_2: *int, var_3: *int

    #node_0:
        store &var_0 <- 10
        store &var_2 <- &var_0
        store &var_1 <- var_2
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for references *)
let%expect_test "ref0" =
  test "./rvalues/ref/ref0.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define ref0::main() : void {
    local var_0: void, var_1: int, var_2: *int

    #node_0:
        store &var_1 <- 10
        n0 = &var_1
        store &var_2 <- n0
        store &var_0 <- null
        ret var_0

    }

  |}]


(* Tests for unary operators *)
let%expect_test "bitwise_not" =
  test "./rvalues/unop/bitwise_not.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define bitwise_not::test_bitwise_not_u32() : void {
      local var_0: void, var_1: int, var_2: int, var_3: int, var_4: void

      #node_0:
          store &var_1 <- 170
          n0 = load &var_1
          n1 = __sil_bnot(n0)
          store &var_2 <- n1
          store &var_0 <- null
          ret var_0
    }

    define bitwise_not::test_bitwise_not_i32() : void {
      local var_0: void, var_1: int, var_2: int, var_3: int, var_4: void

      #node_0:
          store &var_1 <- -1
          n0 = load &var_1
          n1 = __sil_bnot(n0)
          store &var_2 <- n1
          store &var_0 <- null
          ret var_0
    }

    define bitwise_not::main() : void {
      local var_0: void, var_1: void, var_2: void

      #node_0:
          n0 = bitwise_not::test_bitwise_not_u32()
          store &var_1 <- null
          n1 = bitwise_not::test_bitwise_not_i32()
          store &var_2 <- null
          store &var_0 <- null
          ret var_0
    }

  |}]


let%expect_test "bitwise_not" =
  test "./rvalues/unop/bitwise_not.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define bitwise_not::test_bitwise_not_u32() : void {
      local var_0: void, var_1: int, var_2: int, var_3: int, var_4: void

      #node_0:
          store &var_1 <- 170
          n0 = load &var_1
          n1 = __sil_bnot(n0)
          store &var_2 <- n1
          store &var_0 <- null
          ret var_0
    }

    define bitwise_not::test_bitwise_not_i32() : void {
      local var_0: void, var_1: int, var_2: int, var_3: int, var_4: void

      #node_0:
          store &var_1 <- -1
          n0 = load &var_1
          n1 = __sil_bnot(n0)
          store &var_2 <- n1
          store &var_0 <- null
          ret var_0
    }

    define bitwise_not::main() : void {
      local var_0: void, var_1: void, var_2: void

      #node_0:
          n0 = bitwise_not::test_bitwise_not_u32()
          store &var_1 <- null
          n1 = bitwise_not::test_bitwise_not_i32()
          store &var_2 <- null
          store &var_0 <- null
          ret var_0
    }

  |}]


let%expect_test "logical_not" =
  test "./rvalues/unop/logical_not.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define logical_not::main() : void {
      local var_0: void, var_1: bool, var_2: bool, var_3: bool, var_4: void

      #node_0:
          store &var_1 <- true
          n0 = load &var_1
          n1 = __sil_lnot(n0)
          store &var_2 <- n1
          store &var_0 <- null
          ret var_0
    }
  |}]


let%expect_test "neg_int" =
  test "./rvalues/unop/neg_int.ullbc" ;
  [%expect
    {|
    .source_language = "Rust"

    define neg_int::main() : void {
      local var_0: void, var_1: int, var_2: int

      #node_0:
          store &var_1 <- 1
          n0:int = load &var_1
          n1 = __sil_neg(n0)
          store &var_2 <- n1
          store &var_0 <- null
          ret var_0
    }
  |}]
