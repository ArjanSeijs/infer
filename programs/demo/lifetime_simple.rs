fn dangling() -> *const i32 {
    let ten = 10;
    let ptr = &ten;
    &*ptr
}

fn main() {
    unsafe {
        let _undefined = *dangling();
    }
}