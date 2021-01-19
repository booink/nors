extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
use nors::nors::{CounterType, Nors};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("default_counter", |b| {
        b.iter(|| {
            let _ = Nors::new("small.csv").count();
        })
    });
    c.bench_function("bytes_counter", |b| {
        b.iter(|| {
            let _ = Nors::new("small.csv").count_by_type(CounterType::Bytes);
        })
    });
    c.bench_function("csv_crate_counter", |b| {
        b.iter(|| {
            let _ = Nors::new("small.csv").count_by_type(CounterType::CSVCrate);
        })
    });
    c.bench_function("fill_buffer_counter", |b| {
        b.iter(|| {
            let _ = Nors::new("small.csv").count_by_type(CounterType::FillBuffer);
        })
    });
    c.bench_function("lines_counter", |b| {
        b.iter(|| {
            let _ = Nors::new("small.csv").count_by_type(CounterType::Lines);
        })
    });
    c.bench_function("read_line_counter", |b| {
        b.iter(|| {
            let _ = Nors::new("small.csv").count_by_type(CounterType::ReadLine);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
