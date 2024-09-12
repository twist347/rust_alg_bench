use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon::prelude::*;

const START: usize = 250_000;
const FINISH: usize = 1_000_000;
const STEP: usize = 250_000;

const MIN_VAL: i32 = -10_000;
const MAX_VAL: i32 = 10_000;

fn std_sort<T: Ord>(data: &mut [T]) {
    data.sort();
}

fn std_sort_unstable<T: Ord>(data: &mut [T]) {
    data.sort_unstable();
}

fn par_sort<T: Ord + Send>(data: &mut [T]) {
    data.par_sort();
}

fn par_sort_unstable<T: Ord + Send>(data: &mut [T]) {
    data.par_sort_unstable();
}

fn benchmark_sorting_functions(c: &mut Criterion) {
    for size in (START..=FINISH).step_by(STEP) {
        let data: Vec<i32> = (0..size)
            .map(|_| rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL))
            .collect();

        let sort_msg = format!("std::sort of size {}", size);
        c.bench_function(&sort_msg, |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                std_sort(black_box(&mut data_clone))
            })
        });

        let sort_unstable_msg = format!("std::sort_unstable of size {}", size);
        c.bench_function(&sort_unstable_msg, |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                std_sort_unstable(black_box(&mut data_clone))
            })
        });

        let par_sort_msg = format!("rayon_par_sort of size {}", size);
        c.bench_function(&par_sort_msg, |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                par_sort(black_box(&mut data_clone))
            })
        });

        let par_sort_unstable_msg = format!("rayon_par_sort_unstable of size {}", size);
        c.bench_function(&par_sort_unstable_msg, |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                par_sort_unstable(black_box(&mut data_clone))
            })
        });
    }
}

criterion_group!(benches, benchmark_sorting_functions);
criterion_main!(benches);
