struct Wrapper {
    val: String,
}

fn test() {
    let w = Wrapper { val: String::from("hi") };
    let s = w.val;
}
