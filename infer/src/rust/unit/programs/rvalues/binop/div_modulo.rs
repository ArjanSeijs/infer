fn divi_and_mod_signed_unsigned() {
    assert_eq!(7_u32 / 3_u32, 2);
    assert_eq!(7_u32 % 3_u32, 1);

    assert_eq!(7_i32 / -3_i32, -2);
    assert_eq!(7_i32 % -3_i32, 1);
    assert_eq!(-7_i32 / 3_i32, -2);
    assert_eq!(-7_i32 % 3_i32, -1);
}

#[should_panic]
fn divide_by_zero_panics() {
    let _ = 1 / 0;
}

fn divf_nan_inf() {
    let z = 0.0f64;
    let one = 1.0f64;
    let inf = one / z;
    assert!(inf.is_infinite() && inf.is_sign_positive());

    let nan = z / z;
    assert!(nan.is_nan());
}

fn main() {
    divi_and_mod_signed_unsigned();
    divide_by_zero_panics();
    divf_nan_inf();
}
