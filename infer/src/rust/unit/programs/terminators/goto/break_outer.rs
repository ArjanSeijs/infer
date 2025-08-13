pub fn goto_with_break_outer(mut a: i32, mut b: i32) -> i32 {
    'outer: loop {
        loop {
            if a > b {
                break 'outer;
            }
            b += 1;
        }
    }
    a + b
}

fn main() {
    let x: i32 = 0;
    let y: i32 = 1;
    let _ = goto_with_break_outer(x, y);
}