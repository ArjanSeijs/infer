#[allow(unused)]
fn main() {
    let p = Point { x: 1, y: 2 };
    let result = distance(p);

    unsafe {
        let null = maynull(&result);
        let error = *null;
    }
}

fn maynull(i: &i32) -> *const i32 {
    if *i > 10 {
        return 0 as *const i32;
    } else {
        return i as *const i32;
    }
}

fn distance(p: Point) -> i32 {
    p.x * p.x + p.y * p.y
}

struct Point {
    x: i32,
    y: i32,
}
