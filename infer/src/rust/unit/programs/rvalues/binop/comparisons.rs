fn comparisons_ints_and_128() {
    assert!(3_i32 < 5_i32);
    assert!(5_u64 >= 5_u64);
    assert!(i128::MAX > 0);
    assert!(u128::MAX > 0);
    assert!(10_i32 != -10_i32);
}

fn comparisons_floats_with_nan() {
    let nan = f32::NAN;
    assert!(!(nan < 1.0));
    assert!(!(nan > 1.0));
    assert!(!(nan == nan));
    assert!(nan != nan);
}

fn main() {
    comparisons_ints_and_128();
    comparisons_floats_with_nan();
}
