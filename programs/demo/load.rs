//!! --skip-borrowck
#[allow(unused)]
fn foo() -> i32 {
    let ten = 10;
    let tien = ten;
    let a = &tien;
    *a = 20;
    let twenty = *a;
    twenty
}

fn main() {

}