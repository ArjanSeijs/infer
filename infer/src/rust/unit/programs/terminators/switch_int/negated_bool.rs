pub fn swi_not(b: bool) -> i32 {
    if !b { 10 } else { 20 }
}

fn main() {
    let x: bool = false;
    let _ = swi_not(x);
}