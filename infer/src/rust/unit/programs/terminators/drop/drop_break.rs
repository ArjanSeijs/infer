pub fn drop_term_break_local(mut n: i32) -> i32 {
    loop {
        let s = String::from("hi");
        if n > 0 {
            break;
        }
        n -= 1;
    }
    n
}

fn main() {
    let x: i32 = 0;
    let _ = drop_term_break_local(x);
}
