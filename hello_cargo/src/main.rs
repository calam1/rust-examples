
fn main() {
    let mut val = "Hello".to_string();
    // error: cannot borrow `val` as mutable because it is also borrowed as immutable
    let mut val_ref = &val;
    val_ref.push_str(" World");

}