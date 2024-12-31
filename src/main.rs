#![allow(dead_code)]
mod aoc1;
mod aoc2;
mod aoc3;
mod dsu;
mod helpers;

fn main() {
    // let _ = aoc1::aoc1_solve();
    // let _ = aoc2::aoc2_solve();
    let _ = aoc3::aoc3_solve();
}

fn ref_deref() {
    let mut x = 4;
    // y has a mutable reference to x
    let y = &mut x;
    // we can deref and assign a new value
    *y = 5;
}
