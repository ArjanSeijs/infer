#[allow(unused)]
fn main() {
    let x = (1001, 1002);
    let y = ("1003", "1004");
    let one = x.0;
    let two = x.1;
    let (one, two) = x;
    let (three, four) = y;
}