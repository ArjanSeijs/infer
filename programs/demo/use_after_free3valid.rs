fn pointer() -> Box<i32> {
    Box::new(50)
}

fn fifty() -> i32 {
    let result = pointer();
    *result
}

#[allow(unused)]
fn main() {
    let value = fifty();
}
