fn example1(x: &mut i32, y: &mut i32) {
    *x = 42;
    *y = 13;
}

fn main() {
    let mut local = 5;
    let raw = &mut local as *mut i32;
    unsafe {
        let a = &mut *raw;
        let b = &mut *raw;
        example1(a, b)
    };
}
