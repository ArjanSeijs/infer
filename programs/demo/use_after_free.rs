//!! --exclude=core --start-from=use_after_free::main --include=core::mem
fn main() {
    let x = Box::new(50); // x -> a, a -> 50
    let ptr: *const i32 = &*x; //x -> a, ptr -> a, a -> 50
    drop(x); //a -/-> , ptr -> a
    unsafe {
        let __undefined__ = *ptr;
    }
}
