struct Person {
    age: i32,
    height: i32,
    weight: i32,
}

unsafe fn get_person() -> *mut Person {
    let mut person = Person { age: 42, height: 10, weight: 90 };
    &mut person
}

unsafe fn foo() {
    let p : *mut Person;
    p = get_person();
    (*p).age = 1;
}

unsafe fn bar() -> *mut Person {
    let pA: *mut Person;
    {
        let mut personA = Person { age: 42, height: 10, weight: 90 };
        pA = &mut personA;
    }
    pA
}

unsafe fn barbar() {
    let pB = bar();
    (*pB).age = 2;
    //println!("{}", (*p).age);
}

fn main() {
    unsafe {
        barbar();
    }
}
