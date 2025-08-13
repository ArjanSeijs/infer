pub fn swi_parity(x: i32) -> i32 {
    if x % 2 == 0 { x } else { -x }
}

fn main() {
    let x: i32 = 0;
    let _ = swi_parity(x);
}