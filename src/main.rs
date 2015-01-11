use std::cmp;
use std::io;
use std::io::fs::PathExtensions;
use std::iter;
use std::mem;
use std::num;
use std::os;

fn main() {
    for arg in os::args()[1..].iter() {
        let path = Path::new(arg);
        let mut file = io::File::open(&path).unwrap();
        let nbytes: usize = num::from_u64(path.stat().unwrap().size).unwrap();
        let nitems = nbytes / mem::size_of::<u64>();

        let mut u64s = Vec::with_capacity(nitems);
        loop {
            let u64 = match file.read_le_u64() {
                Ok(h) => h,
                Err(_) => break,
            };
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
}
