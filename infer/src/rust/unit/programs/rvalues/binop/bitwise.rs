fn bitwise_ops() {
    assert_eq!(0b1010u8 & 0b1100u8, 0b1000u8);
    assert_eq!(0b1010u8 | 0b0101u8, 0b1111u8);
    assert_eq!(0b1111u8 ^ 0b0101u8, 0b1010u8);

    let a: i8 = -1;
    assert_eq!(a & 0b0000_1111i8, 0b0000_1111i8);
}

fn main() {
    bitwise_ops();
}