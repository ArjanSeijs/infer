#[allow(unused)]
fn main() {
    let p = Point { x: 3, y: 4 };
    let result = distance(&p);
}

#[allow(unused)]
fn distance(p: &Point) -> i32 {
    p.x * p.x + p.y * p.y
}

struct Point {
    x: i32,
    y: i32,
}
