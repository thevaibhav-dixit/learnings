#![allow(dead_code)]
mod dsu;

fn main() {
    {}
}

fn ref_deref() {
    let mut x = 4;
    // y has a mutable reference to x
    let y = &mut x;
    // we can deref and assign a new value
    *y = 5;
}
