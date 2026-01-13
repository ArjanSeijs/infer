#[allow(unused)]
fn use_evil_pointer() {
    let evil_pointer = create_evil_pointer();
    unsafe { *evil_pointer = 10 };
}

#[allow(unused)]
fn create_evil_pointer() -> *mut i32 {
    let mut x = 42;
    let x_ref = &mut x;
    let evil_pointer = x_ref as *mut i32;
    return evil_pointer;
}

fn main() {
    // create_evil_pointer();
}