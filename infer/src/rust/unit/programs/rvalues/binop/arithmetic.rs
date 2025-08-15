fn plus_minus_mult_basic() {
    let a: i32 = -7;
    let b: i32 = 3;
    assert_eq!(a + b, -4);
    assert_eq!(a - b, -10);
    assert_eq!(a * b, -21);

    let x: u32 = 2;
    let y: u32 = 5;
    assert_eq!(x + y, 7);
    assert_eq!(y - x, 3);
    assert_eq!(x * y, 10);

    let t: bool = true;  
    let f: bool = false; 

    assert_eq!((t as u8) + (f as u8), 1);
    assert_eq!((t as u8) * (t as u8), 1);

    let c: char = 'A';
    assert_eq!((c as u32) + 1, 66);
}

fn arithmetic_edges_i128_u128() {
    let max_u128 = u128::MAX;
    assert_eq!(max_u128.wrapping_add(1), 0);

    let min_i128 = i128::MIN;

    assert_eq!(min_i128.wrapping_sub(1), i128::MAX);
}

fn main() {
    plus_minus_mult_basic();
    arithmetic_edges_i128_u128();
}