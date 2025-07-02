fn add(a: i32, b: i32) -> i32 {
    let c = a + b + 42;
    let d = c + 1;
    d + 129
}

fn main() {
    let _x = add(1, 2);
}

/*
fn add(_1: i32, _2: i32) -> i32 {
    debug a => _1;
    debug b => _2;
    let mut _0: i32;
    let _3: i32;
    let mut _4: i32;
    scope 1 {
        debug c => _3;
        let _5: i32;
        scope 2 {
            debug d => _5;
        }
    }

    bb0: {
        StorageLive(_4);
        _4 = Add(copy _1, copy _2);
        _3 = Add(move _4, const 42_i32);
        StorageDead(_4);
        _5 = Add(copy _3, const 1_i32);
        _0 = Add(copy _5, const 129_i32);
        return;
    }
}

fn main() -> () {
    let mut _0: ();
    scope 1 (inlined add) {
        scope 2 {
            scope 3 {
            }
        }
    }

    bb0: {
        return;
    }
}
*/