use test_crate::*;

enum NeedDefault {
    A,
    B,
}

append_dummy!(need_default);

fn main() {
    let _ = NeedDefault::default();
}
