fn main() {
    let ptr: *const i32 = std::ptr::null(); // ptr -> a, a = null

    unsafe {
        let _ : i32 = *ptr; //Panic null pointer derefernce 
    }
}