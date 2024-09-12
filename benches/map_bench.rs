use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon::prelude::*;

const START: usize = 100_000;
const FINISH: usize = 500_000;
const STEP: usize = 100_000;

const MIN_VAL: f64 = -10_000.0;
const MAX_VAL: f64 = 10_000.0;

fn newton_sqrt(x: f64) -> f64 {
    let eps = 1e-10;
    let n = 1_000;
    let mut guess = x;

    for _ in 0..n {
        let f = guess * guess - x;
        let f_prime = 2.0 * guess;
        let next_guess = guess - f / f_prime;
        if (next_guess - guess).abs() < eps {
            break;
        }
        guess = next_guess;
    }

    guess
}

fn map(data: &[f64]) -> Vec<f64> {
    data.iter().map(|&x| newton_sqrt(x)).collect()
}

fn par_map(data: &[f64]) -> Vec<f64> {
    data.par_iter().map(|&x| newton_sqrt(x)).collect()
}

fn bench_map(c: &mut Criterion) {
    for size in (START..=FINISH).step_by(STEP) {
        let data: Vec<f64> = (0..size)
            .map(|_| rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL))
            .collect();

        let map_msg = format!("map of size {}", size);
        c.bench_function(&map_msg, |b| {
            b.iter(|| map(black_box(&data)))
        });
    }
}

fn bench_par_map(c: &mut Criterion) {
    for size in (START..=FINISH).step_by(STEP) {
        let data: Vec<f64> = (0..size)
            .map(|_| rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL))
            .collect();

        let map_msg = format!("par_map of size {}", size);
        c.bench_function(&map_msg, |b| {
            b.iter(|| par_map(black_box(&data)))
        });
    }
}

criterion_group!(benches, bench_map, bench_par_map);
criterion_main!(benches);