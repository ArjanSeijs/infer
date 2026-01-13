//!! --exclude=core --exclude=alloc --start-from=use_after_free3::main --monomorphize --include=i32 --hide-marker-traits
#[allow(unused)]
fn main() {
    unsafe {
        let result = pointer();
        let __undefined__ = *result;
        // println!("{}",*result) // undefined behaviour prints random number
        // Detected by miri at runtime:
        //error: Undefined Behavior: pointer not dereferenceable: alloc236 has been freed, so this pointer is dangling
    }
}

fn pointer() -> *const i32 {
    let x = Box::new(50);
    &*x //Reborrow as raw pointer
    // x goes out of scope and is dropped
}