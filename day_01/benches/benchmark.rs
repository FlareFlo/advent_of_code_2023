use std::fs;
use criterion::{criterion_group, criterion_main, Criterion};
use day_01::solution;
use day_01::solution_complex;

fn bench_simple(c: &mut Criterion) {
	let input = fs::read("input.txt").unwrap();
	c.bench_function("solution day 1 part 1", |b| b.iter(|| solution(&input)));
}

fn bench_complex(c: &mut Criterion) {
	let mut input = fs::read("input.txt").unwrap();
	c.bench_function("solution day 1 part 2", |b| b.iter(|| solution_complex(&mut input)));
}

criterion_group!(benches, bench_simple, bench_complex);
criterion_main!(benches);