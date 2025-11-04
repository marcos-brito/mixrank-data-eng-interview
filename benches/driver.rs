use criterion::{Criterion, criterion_group, criterion_main};
use data_eng_interview::driver;

static INPUT: &str = "\
facebook.com
twitter.com
ask.com
wix.com
indeed.com
godaddy.com
";

pub fn drivers_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("drivers");

    group.sample_size(10);

    group.bench_function("single_thread", |b| {
        b.iter(|| driver::single_thread(INPUT.as_bytes()));
    });

    group.bench_function("fork_join", |b| {
        b.iter(|| driver::fork_join(INPUT.as_bytes()));
    });

    group.bench_function("worker_pool", |b| {
        b.iter(|| driver::worker_pool(32, INPUT.as_bytes()));
    });

    group.finish();
}

pub fn pool_size_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_size");

    group.sample_size(10);

    group.bench_function("16 workers", |b| {
        b.iter(|| driver::worker_pool(16, INPUT.as_bytes()));
    });

    group.bench_function("32 workers", |b| {
        b.iter(|| driver::worker_pool(32, INPUT.as_bytes()));
    });

    group.bench_function("64 workers", |b| {
        b.iter(|| driver::worker_pool(64, INPUT.as_bytes()));
    });

    group.finish();
}

criterion_group!(benches, drivers_benchmark, pool_size_benchmark);
criterion_main!(benches);
