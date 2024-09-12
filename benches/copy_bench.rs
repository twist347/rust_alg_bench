use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon::prelude::*; // Не забудьте добавить rayon в Cargo.toml

const START: usize = 25_000_000;
const FINISH: usize = 100_000_000;
const STEP: usize = 25_000_000;

const MIN_VAL: i32 = -10_000;
const MAX_VAL: i32 = 10_000;

fn slice_copy<T: Copy>(src: &[T], dst: &mut [T]) {
    dst.copy_from_slice(src); 
}

fn parallel_copy(src: &[i32], dst: &mut [i32]) {
    if src.len() != dst.len() {
        panic!("Source and destination slices must have the same length.");
    }

    dst.par_iter_mut()
        .zip(src.par_iter())
        .for_each(|(dst, src)| *dst = *src);
}

fn benchmark_copy_funcs(c: &mut Criterion) {
    for size in (START..=FINISH).step_by(STEP) {
        let src: Vec<i32> = (0..size)
            .map(|_| rand::thread_rng().gen_range(MIN_VAL..=MAX_VAL))
            .collect();

        let slice_copy_msg = format!("slice_copy of size {}", size);
        c.bench_function(&slice_copy_msg, |b| {
            b.iter(|| {
                let mut dst = vec![0; size];
                slice_copy(black_box(&src), black_box(&mut dst));
            })
        });

        let par_copy_msg = format!("slice_copy of size {}", size);
        c.bench_function(&par_copy_msg, |b| {
            b.iter(|| {
                let mut dst = vec![0; size];
                parallel_copy(black_box(&src), black_box(&mut dst));
            })
        });
    }
}

criterion_group!(benches, benchmark_copy_funcs);
criterion_main!(benches);
