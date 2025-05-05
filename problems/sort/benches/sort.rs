use divan::{Bencher, AllocProfiler};
use sort::{merge_sort, insertion_sort, quick_sort};

const SIZES: &[usize] =
    &[1, 2, 8, 16, 64, 512, 4 * 1024, 16 * 1024];

// #[global_allocator]
// static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

mod gen {

    pub fn rand_int_vec_generator(size: usize) -> impl FnMut() -> Vec<i32> {
        move || {
            let mut rng = fastrand::Rng::with_seed(42);
            (0..size).map(|_| rng.i32(..)).collect()
        }
    }

    pub fn sorted_int_vec_generator(size: usize) -> impl FnMut() -> Vec<i32> {
        move || (0..size).map(|i| i as i32).collect()
    }
}
    

mod random {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn insertion_sort_bench(bencher: Bencher, n: usize) {
        // `rand_int_vec_generator` を使用しています。
        bencher.with_inputs(gen::rand_int_vec_generator(n)).bench_local_refs(|v| {
            insertion_sort::sort(v);
        });
    }

    #[divan::bench(args = SIZES)]
    fn merge_sort_bench(bencher: Bencher, n: usize) {
        // `rand_int_vec_generator` を使用しています。
        bencher.with_inputs(gen::rand_int_vec_generator(n)).bench_local_refs(|v| {
            merge_sort::sort(v);
        });
    }

    #[divan::bench(args = SIZES)]
    fn quick_sort_bench(bencher: Bencher, n: usize) {
        // `rand_int_vec_generator` を使用しています。
        bencher.with_inputs(gen::rand_int_vec_generator(n)).bench_local_refs(|v| {
            quick_sort::sort(v);
        });
    }


}

mod sorted {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn insertion_sort_bench(bencher: Bencher, n: usize) {
        bencher.with_inputs(gen::sorted_int_vec_generator(n)).bench_local_refs(|v| {
            insertion_sort::sort(v);
        });
    }

    #[divan::bench(args = SIZES)]
    fn merge_sort_bench(bencher: Bencher, n: usize) {
        bencher.with_inputs(gen::sorted_int_vec_generator(n)).bench_local_refs(|v| {
            merge_sort::sort(v);
        });
    }

    #[divan::bench(args = SIZES)]
    fn quick_sort_bench(bencher: Bencher, n: usize) {
        // `rand_int_vec_generator` を使用しています。
        bencher.with_inputs(gen::sorted_int_vec_generator(n)).bench_local_refs(|v| {
            quick_sort::sort(v);
        });
    }
}
