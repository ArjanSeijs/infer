struct Wrapper {
    val: String,
}

fn main() {
    let w = Wrapper { val: String::from("hi") };
    let s = w.val;
}
