#[allow(unused)]
fn main() {
    let mut p = Point { x: 12, y: 13 };
    // p.x = 1;
    // p.y = 4;
    // let xvalue = p.x;
    let result = distance(p);
    // let mut ptr = &raw mut result;

    // latent_null_dereference(result, &raw mut ptr )
}

#[allow(unused)]
fn set_to_null_if_positive(n : i32, p: *mut *mut i32)
{
    if n > 24
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

// #[allow(unused)]
// fn main2() {
//     let mut p = Point { x: 30, y: 40 };
//     let xxx = p.x * p.x;
//     let yyy = p.y * p.y;
//     let result = xxx + yyy;
// }

// #[allow(unused)]
// fn main3() {
//     let mut p = Point { x: 30, y: 40 };
//     let x = p.x;
//     let y = p.y;
//     let xxx = x * x;
//     let yyy = y * y;
//     let result = xxx + yyy;
// }

// #[allow(unused)]
// fn x(mut p: Point) -> i32 {
//     p.x
// }

#[allow(unused)]
fn distance(mut p: Point) -> i32 {
    // let p = Point { x: 1, y: 4 };
    // p.x * p.x + p.y * p.y
    // p.x = 1;
    // p.y = 2;
    let x = p.x;
    let y = p.y;
    let xx = x * x;
    let yy = y * y;
    xx + yy
}

struct Point {
    x: i32,
    y: i32,
}
