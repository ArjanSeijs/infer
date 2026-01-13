//!! --exclude=core --start-from=use_after_free2::main
fn main() {
    let ptr: *const i32; // ptr -> -
    unsafe {
        {
            let x = Box::new(50); // x -> a, a -> 50
            ptr = &*x; //x -> a, ptr -> a, a -> 50
        }
        // x dropped: ptr -> a, a -/->
        let __undefined__ = *ptr; // Undefined behavior: use after free
    }
}
