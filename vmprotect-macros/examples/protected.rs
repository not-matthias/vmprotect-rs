use std::hint::black_box;
use vmprotect_macros::vmprotect;

#[vmprotect(Ultra, "my_name")]
fn protected_fn() {
    println!("This function is protected");
}

fn main() {
    println!("Hello World");
    protected_fn();
}
