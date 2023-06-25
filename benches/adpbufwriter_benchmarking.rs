use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::BufWriter;

// this will generate random data of 10MB to be written to the file
fn generate_data() -> Vec<u8> {
    let mut data = Vec::with_capacity(10 * 1024 * 1024);
    for _ in 0..10 * 1024 * 1024 {
        data.push(rand::random::<u8>());
    }
    data
}

fn bufwriter8kb_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("BufWriter");
    group.sample_size(10);
    group.bench_function("BufWriter 8kb", |b| {
        b.iter(|| {
            let dst = "/Volumes/PNY 2/adpbufwriter_test_dst/bufwriter.txt";
            let file = std::fs::File::create(dst).unwrap();
            let data = generate_data();
            let mut writer = BufWriter::new(file);
            std::io::copy(&mut data.as_slice(), &mut writer).unwrap();
        })
    });
    group.finish();
}

fn bufwriter16kb_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("BufWriter");
    group.sample_size(10);
    group.bench_function("BufWriter 16Kb", |b| {
        b.iter(|| {
            let dst = "/Volumes/PNY 2/adpbufwriter_test_dst/bufwriter.txt";
            let file = std::fs::File::create(dst).unwrap();
            let data = generate_data();
            let mut writer = BufWriter::with_capacity(16 * 1024, file);
            std::io::copy(&mut data.as_slice(), &mut writer).unwrap();
        })
    });
    group.finish();
}

fn adpbufwriter_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ADP Buf Writer");
    group.sample_size(10);
    group.bench_function("Adp Buf Writer", |b| {
        b.iter(|| {
            let dst = "/Volumes/PNY 2/adpbufwriter_test_dst/adpbufwriter.txt";
            let file = std::fs::File::create(dst).unwrap();
            let data = generate_data();
            let mut writer = BufWriter::with_capacity(16 * 1024, file);
            std::io::copy(&mut data.as_slice(), &mut writer).unwrap();
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bufwriter8kb_benchmark,
    bufwriter16kb_benchmark,
    // adpbufwriter_benchmark
);
criterion_main!(benches);
