fn test_bitwise_not_u32() {
    let x: u32 = 0b1010_1010;
    let y = !x;
}

fn test_bitwise_not_i32() {
    let x: i32 = -1;
    let y = !x;
}

fn main() {
    test_bitwise_not_u32();
    test_bitwise_not_i32();
}