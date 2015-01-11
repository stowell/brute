use std::cmp;
use std::io;
use std::io::fs::PathExtensions;
use std::iter;
use std::num;
use std::os;

fn main() {
    for arg in os::args()[1..].iter() {
        let path = Path::new(arg);
        let mut file = io::File::open(&path).unwrap();
        let nbytes: usize = num::from_u64(path.stat().unwrap().size).unwrap();
        let nitems = nbytes / 20;

        let mut ids_listeners = Vec::with_capacity(nitems);
        loop {
            let high = match file.read_le_u64() {
                Ok(h) => h,
                Err(_) => break,
            };
            let low = match file.read_le_u64() {
                Ok(l) => l,
                Err(_) => break,
            };
            let listeners = match file.read_le_u32() {
                Ok(l) => l,
                Err(_) => break,
            };
            ids_listeners.push((listeners, (high, low)));
        }

        const STEP: usize = 4;
        for start in iter::range_step(0, nitems, STEP) {
            let end = cmp::min(start + STEP, nitems);
            let view = ids_listeners.slice_mut(start, end);
            view.sort();
        }

        for item in ids_listeners.iter() {
            let (listeners, (high, low)) = *item;
            println!("({}, {}) has {} listeners", high, low, listeners)
        }
    }
}
