pub fn fn_complex(mut a: i32, mut b: i32) -> i32 {
    let s = String::from("Start");
    for _i in 0..3 {
        match (a + b) % 4 {
            0 => {
                a += helper(a);
            }
            1 => {
                b += helper(b);
                drop(s.clone());
            }
            2 => {
                a -= 1;
                if a < 0 { break; }
            }
            _ => {
                b -= 1;
                if b < 0 { continue; }
            }
        }
    }
    a + b
}

fn helper(x: i32) -> i32 {
    x * 2
}

fn main() {
    let x: i32 = 0;
    let y: i32 = 1;
    let _ = fn_complex(x, y);
}
