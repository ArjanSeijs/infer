fn add(a: i32, b: i32) -> i32 {
    let c = a + b;
    let d = c + 1;
    d + 129
}

fn main() {
    add(1, 2);
}

/*
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
// HINT: See also -Z dump-mir for MIR at specific points during compilation.
fn add(_1: i32, _2: i32) -> i32 {
    debug a => _1;
    debug b => _2;
    let mut _0: i32;
    let _3: i32;
    scope 1 {
        debug c => _3;
        let _4: i32;
        scope 2 {
            debug d => _4;
        }
    }

    bb0: {
        _3 = Add(copy _1, copy _2);
        _4 = Add(copy _3, const 1_i32);
        _0 = Add(copy _4, const 129_i32);
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