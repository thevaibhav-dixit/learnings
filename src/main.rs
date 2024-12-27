#![allow(dead_code)]
mod aoc1;
mod dsu;

fn main() {
    let _ = aoc1::solve();
}

fn ref_deref() {
    let mut x = 4;
    // y has a mutable reference to x
    let y = &mut x;
    // we can deref and assign a new value
    *y = 5;
}
