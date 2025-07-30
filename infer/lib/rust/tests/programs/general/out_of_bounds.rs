fn main() {
    let arr = [1, 2, 3];
    let ptr = &arr as *const i32;
    // let ptr = as_ptr::<i32>(x);
    unsafe {
        let ptr = ptr.add(50);
        let _ = *ptr;
        // println!("{}", *ptr); //undefined behaviour: ~2^15
        
    }
}