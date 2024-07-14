use testing_code::sorting::{sort_algo_1, sort_algo_2};

use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![
        64, 34, 25, 12, 22, 11, 90, 28, 33, 47, 32, 55, 31, 88, 57, 41, 75, 85, 59, 62, 65, 43, 77,
        19, 97, 29, 89, 53, 91, 86, 14, 51, 66, 79, 37, 48, 56, 82, 67, 98, 73, 92, 69, 80, 70, 54,
        60, 23, 39, 15, 18, 95, 99, 76, 44, 58, 50, 49, 38, 87, 26, 93, 45, 17, 94, 72, 35, 63, 83,
        27, 40, 81, 13, 16, 61, 20, 74, 36, 68, 78, 46, 52, 84, 21, 30, 42, 96, 24, 100, 71, 3, 8,
        2, 10, 1, 7, 4, 6, 5, 9,
    ];
    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
