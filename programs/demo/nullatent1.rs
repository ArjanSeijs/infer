//!! --exclude=core --start-from=nullatent1::main --include=core::ptr
#[allow(unused)]
fn set_to_null_if_positive(n : i32, p: *mut *mut i32)
{
    if n > 0
    {
        unsafe {
            *p = std::ptr::null_mut();
        }
    }
}

#[allow(unused)]
fn latent_null_dereference(n : i32, p: *mut *mut i32)
{
    set_to_null_if_positive(n, p);
    unsafe {
        **p = 10; // NULL dereference! but only if n > 0 so no report yet    
    }
}

fn main() {
    let mut x = -20;
    let pointer = &mut(&mut x as *mut i32) as *mut *mut i32;
    latent_null_dereference(x,pointer)
}