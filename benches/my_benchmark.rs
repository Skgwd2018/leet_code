use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 被测试的函数
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
// time:   [17.235 µs 17.297 µs 17.361 µs]

// 被测试的函数(优化后)
fn fibonacci2(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
// time:   [4.9158 ns 4.9403 ns 4.9663 ns]

pub fn fibonacci3(n: usize) -> usize {
    if n < 2 { return n; }

    // let mut dp = vec![0; n + 1];
    // dp[0] = 0;
    // dp[1] = 1;
    // for i in 2..=n {
    //     dp[i] = dp[i - 1] + dp[i - 2];
    // }
    // dp[n]
    // time:   [71.225 ns 71.475 ns 71.772 ns]

    (2..=n).fold((0, 1), |(dp1, dp2), _| (dp2, dp1 + dp2)).1
}
// time:   [6.4998 ns 6.6538 ns 6.8004 ns]

// 基准测试函数
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci2(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// 执行所有基准测试
// cargo bench

// 执行指定基准测试
// cargo bench --bench my_benchmark
// 使用参考: https://bheisler.github.io/criterion.rs/book/getting_started.html