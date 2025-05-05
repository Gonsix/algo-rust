use divan::{Bencher, AllocProfiler};
use search::{linear, binary};
use std::{
    collections::{hash_map::RandomState, BTreeSet, HashSet},
    hash::BuildHasher,
};
use fastrand::Rng;

const SIZES: &[usize] = &[1, 2, 8, 16, 64, 512, 4 * 1024, 16 * 1024, 16 * 16 * 1024, 1024 * 1024];


fn main () {
    divan::main();
}

fn gen_inputs(len: usize) -> impl FnMut() -> (Vec<u64>, u64) {
    let mut rng = Rng::with_seed(len as u64);

    move || {
        let haystack: Vec<u64> = {
            // Use `BTreeSet` to ensure result is sorted and has `len` items.
            let mut haystack = BTreeSet::new();

            for _ in 0..len {
                while !haystack.insert(rng.u64(..)) {}
            }

            haystack.into_iter().collect()
        };

        let has_needle = rng.bool();
        let needle = if has_needle {
            *rng.choice(&haystack).unwrap()
        } else {
            loop {
                let n = rng.u64(..);
                if !haystack.contains(&n) {
                    break n;
                }
            }
        };

        assert_eq!(haystack.len(), len);
        (haystack, needle)
    }
}

#[divan::bench(args = SIZES, max_time = 1)]
fn linear_bench(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_inputs(len))
        .bench_local_refs(|(haystack, needle)| linear::linear_search(haystack, *needle))
}


#[divan::bench(args = SIZES, max_time = 1)]
fn binary_bench(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_inputs(len))
        .bench_local_refs(|(haystack, needle)| binary::binary_search(haystack, *needle))
}
