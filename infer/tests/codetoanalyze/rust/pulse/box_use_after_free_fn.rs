#![allow(unused)]
fn main() {
    let ptr = freed();
    let ub = unsafe {*ptr}; // Error Occurs Here
}

fn freed() -> *const i32 {
    let x = Box::new(50);
    &*x // Convert box to raw pointer
    // Box is freed here since borrowchecker does not keep track of raw pointer
}