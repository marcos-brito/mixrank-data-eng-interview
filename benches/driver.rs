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

    group.finish();
}

criterion_group!(benches, drivers_benchmark);
criterion_main!(benches);
