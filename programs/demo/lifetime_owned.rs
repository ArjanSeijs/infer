struct Person {
    age: i32,
    height: i32,
    weight: i32,
}

fn get_person() -> Person {
    let person = Person { age: 42, height: 10, weight: 90 };
    person
}

fn foo() {
    let mut p : Person;
    p = get_person();
    p.age = 1;
}

fn main() {
    foo();
}
