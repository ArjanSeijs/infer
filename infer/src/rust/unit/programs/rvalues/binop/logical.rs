fn side_effect(counter: &mut i32) -> bool {
    *counter += 1;
    true
}

fn logical_and_short_circuit() {
    let mut c = 0;
    let result = false && side_effect(&mut c);
    assert!(!result);
    assert_eq!(c, 0);

    let result2 = true && side_effect(&mut c);
    assert!(result2);
    assert_eq!(c, 1);
}

fn logical_or_short_circuit() {
    let mut c = 0;
    let result = true || side_effect(&mut c);
    assert!(result);
    assert_eq!(c, 0);

    let result2 = false || side_effect(&mut c);
    assert!(result2);
    assert_eq!(c, 1);
}

fn main() {
    logical_and_short_circuit();
    logical_or_short_circuit();
}