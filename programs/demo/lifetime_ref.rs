//!! --skip-borrowck
struct Person {
    age: i32,
    height: i32,
    weight: i32,
}

fn get_person<'a>() -> &'a mut Person {
    let mut person = Person { age: 42, height: 10, weight: 90 };
    &mut person
}

fn foo() {
    let p : &mut Person;
    p = get_person();
    (*p).age = 1;
}

fn bar<'a>() -> &'a mut Person {
    let pA: &mut Person;
    {
        let mut personA = Person { age: 42, height: 10, weight: 90 };
        pA = &mut personA;
    }
    pA
}

fn barbar() {
    let pB = bar();
    (*pB).age = 2;
    //println!("{}", (*p).age);
}

fn main() {
    barbar();
}
