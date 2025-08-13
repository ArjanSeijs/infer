pub fn drop_term_continue_local(mut n: i32) -> i32 {
    loop {
        let s = String::from("x");
        if n % 2 == 0 {
            n += 1;
            continue;
        }
        n += 1;
        if n > 10 { break; }
    }
    n
}

fn main() {
    let x: i32 = 0;
    let _ = drop_term_continue_local(x);
}
