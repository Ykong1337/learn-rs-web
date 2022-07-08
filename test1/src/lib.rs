use std::mem;

#[test]
fn a() {
    let a;
    let a = a = true;
    print!("{}", mem::size_of_val(&a));
}
