use std::fs;
use criterion::{criterion_group, criterion_main, Criterion};
use day_01::solution;

fn bench_simple(c: &mut Criterion) {
	let input = fs::read_to_string("input.txt").unwrap();
	c.bench_function("solution day 1 part 1", |b| b.iter(|| solution(&input)));
}

criterion_group!(benches, bench_simple);
criterion_main!(benches);