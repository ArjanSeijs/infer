//!! --exclude=core --start-from=nullsimple::main --include=core::ptr
#[allow(unused_variables)]
fn main() {
    // Set Wijzer
    let wijzer: *const i32 = std::ptr::null(); // ptr -> a, a-> null

    unsafe {
        // Set X
        let _x : i32 = *wijzer; //Panic null pointer derefernce
    }
}
