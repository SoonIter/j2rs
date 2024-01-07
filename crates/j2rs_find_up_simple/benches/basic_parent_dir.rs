use std::path::PathBuf;

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};

fn j2rs_find_up_simple() -> PathBuf {
  j2rs_find_up_simple::find_up("Cargo.lock").unwrap()
}

fn lets_find_up() -> PathBuf {
  lets_find_up::find_up("Cargo.lock").unwrap().unwrap()
}

fn main_bench(b: &mut Criterion) {
  let mut group = b.benchmark_group("basic_parent_dir_benchmark");
  group.sample_size(10);
  group.bench_function("j2rs_find_up_simple", |b| b.iter(j2rs_find_up_simple));
  group.bench_function("lets_find_up", |b| b.iter(lets_find_up));
}

criterion_group!(benches, main_bench);
criterion_main!(benches);
