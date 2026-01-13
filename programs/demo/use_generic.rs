// Example of disamubgation by charon

fn id<T> (t :&T) -> &T {
    t
}

fn main() {
    id(&10);
}