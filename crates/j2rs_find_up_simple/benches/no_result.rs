use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};

fn j2rs_find_up_simple() -> Option<PathBuf> {
  j2rs_find_up_simple::find_up("NoSuchFileAndNoResult.txt")
}

fn lets_find_up() -> std::io::Result<Option<PathBuf>> {
  lets_find_up::find_up("NoSuchFileAndNoResult.txt")
}

fn main_bench(b: &mut Criterion) {
  let mut group = b.benchmark_group("no_result_benchmark");
  group.sample_size(10);
  group.bench_function("j2rs_find_up_simple_no_result", |b| b.iter(j2rs_find_up_simple));
  group.bench_function("lets_find_up_no_result", |b| b.iter(lets_find_up));
}

criterion_group!(benches, main_bench);
criterion_main!(benches);
