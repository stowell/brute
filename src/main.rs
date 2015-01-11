use std::cmp;
use std::iter;
use std::rand;

fn main() {
    let nitems = 1024;
    let mut u64s = Vec::with_capacity(nitems);
    for _ in range(0, nitems) {
        let u64: u64 = rand::random();
        u64s.push(u64);
    }

    const STEP: usize = 4;
    for start in iter::range_step(0, nitems, STEP) {
        let end = cmp::min(start + STEP, nitems);
        let view = u64s.slice_mut(start, end);
        view.sort();
    }

    for &u64 in u64s.iter() {
        println!("{}", u64);
    }
}
