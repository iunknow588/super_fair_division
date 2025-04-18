use criterion::{black_box, criterion_group, criterion_main, Criterion};
use super_fair_division::calculate_fair_division;

fn bench_small_input(c: &mut Criterion) {
    let small_input = vec![1, 2, 3, 4, 5];
    c.bench_function("small input", |b| {
        b.iter(|| calculate_fair_division(black_box(&small_input)))
    });
}

fn bench_medium_input(c: &mut Criterion) {
    let medium_input: Vec<i128> = (1..100).collect();
    c.bench_function("medium input", |b| {
        b.iter(|| calculate_fair_division(black_box(&medium_input)))
    });
}

fn bench_large_input(c: &mut Criterion) {
    let large_input: Vec<i128> = (1..1000).collect();
    c.bench_function("large input", |b| {
        b.iter(|| calculate_fair_division(black_box(&large_input)))
    });
}

criterion_group!(benches, bench_small_input, bench_medium_input, bench_large_input);
criterion_main!(benches);
