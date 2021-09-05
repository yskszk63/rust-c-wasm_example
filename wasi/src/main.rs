extern "C" {
    fn chello();
}

fn main() {
    unsafe { chello() }
}
