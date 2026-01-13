//!! --exclude=core --start-from=use_after_free_struct::main --monomorphize --include=i32 --hide-marker-traits --skip-borrowck
struct Val {
    value: i32
}

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

fn pointer() -> *const Val {
    let x = Box::new(Val{value:10});
    &*x //Reborrow as raw pointer
    // x goes out of scope and is dropped
}