fn shifts_signed_vs_unsigned() {
    assert_eq!(1_u8 << 3, 8);
    assert_eq!(0b1000_0000u8 >> 7, 1);

    let x: i8 = -2; 
    assert_eq!(x >> 1, -1); 
}

#[should_panic]
fn shift_too_large_panics_in_debug() {
    let _ = 1u8 << 8;
}

fn main() {
    shifts_signed_vs_unsigned();
    shift_too_large_panics_in_debug();
}