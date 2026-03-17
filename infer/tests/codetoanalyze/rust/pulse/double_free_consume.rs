fn consume(x: Box<i32>) {
    
}

fn main() {
    let val = Box::new(7);
    let ptr = (&*val) as *const i32;
    consume(unsafe {std::mem::transmute(ptr)});
    consume(unsafe {std::mem::transmute(ptr)});
}