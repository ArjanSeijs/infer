fn main() {
    latent(-1)                  // [i <= 0] latent (i) [err : ...]
}

fn latent(i : i32) {            // E.N: i -> N
    let x = 10;                 // E.N: x = 10
    let ptr : *const i32;       // E.N: x = 10 * ptr -> T
    if i > 0  {                 // (1) E.N: x = 10 * ptr -> T * i > 0
        ptr = std::ptr::null(); // (1) E.N: x = 10 * ptr -> P * P = null * i > 0
    } else {                    // (2) E.N: x = 10 * ptr -> T * i <= 0
        ptr = &x;               // (2) E.N: x = 10 * ptr -> x * i <= 0
    }                           //
    unsafe {                    //  
        let _ = *ptr;           // (1) err : ptr -> P * P = null 
    }                           // (2) ok: ptr -> x
                                //
}