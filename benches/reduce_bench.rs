use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon::prelude::*; // Для параллельных итераторов

const START: usize = 1_000_000;
const FINISH: usize = 10_000_000;
const STEP: usize = 1_000_000;

const MIN_VAL: f64 = 0.0;
const MAX_VAL: f64 = 1.0;

fn fold(data: &[f64]) -> f64 {
    data.iter().fold(0.0, |acc, &x| acc + x)
}

fn parallel_fold(data: &[f64]) -> f64 {
    data.par_iter().fold(|| 0.0, |acc, &x| acc + x).reduce(|| 0.0, |a, b| a + b)
}

fn reduce(data: &[f64]) -> f64 {
    data.iter().copied().reduce(|a, b| a + b).unwrap_or(0.0)
}

fn parallel_reduce(data: &[f64]) -> f64 {
    data.par_iter().copied().reduce(|| 0.0, |a, b| a + b)
}

fn benchmark_fold_funcs(c: &mut Criterion) {
    for size in (START..=FINISH).step_by(STEP) {
        let data: Vec<f64> = (0..size)
            .map(|_| rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL))
            .collect();

        let seq_fold_msg = format!("fold of size {}", size);
        c.bench_function(&seq_fold_msg, |b| {
            b.iter(|| fold(black_box(&data)))
        });

        let par_fold_msg = format!("par_fold of size {}", size);
        c.bench_function(&par_fold_msg, |b| {
            b.iter(|| parallel_fold(black_box(&data)))
        });
    }
}

fn benchmark_reduce_funcs(c: &mut Criterion) {
    for size in (START..=FINISH).step_by(STEP) {
        let data: Vec<f64> = (0..size)
            .map(|_| rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL))
            .collect();

        let seq_reduce_msg = format!("reduce of size {}", size);
        c.bench_function(&seq_reduce_msg, |b| {
            b.iter(|| reduce(black_box(&data)))
        });

        let par_reduce_msg = format!("par_reduce of size {}", size);
        c.bench_function(&par_reduce_msg, |b| {
            b.iter(|| parallel_reduce(black_box(&data)))
        });
    }
}

criterion_group!(benches, benchmark_fold_funcs, benchmark_reduce_funcs);
criterion_main!(benches);
