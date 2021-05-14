use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod dayfive;

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn day5_part2(c: &mut Criterion) {
    let input_string = dayfive::parse_input(dayfive::INPUT);
    c.bench_function("day5_part2", |b| b.iter(|| {
        let x = input_string.to_vec();
        dayfive::part_two(x)
    }
    ));


}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, day5_part2);
criterion_main!(benches);