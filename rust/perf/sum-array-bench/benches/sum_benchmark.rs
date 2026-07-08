// Run via 'cargo bench'

use criterion::{criterion_group, criterion_main, Criterion};
pub use sum_array_bench::{sum_array, sum_array_iter, sum_array_rayon, sum_array_simd};

fn bench_sum(c: &mut Criterion)
{
   let arr: Vec<f32> = (0..1000000).map(|x| x as f32).collect();

   c.bench_function("sum_array", |b| b.iter(|| sum_array(&arr)));
   c.bench_function("sum_array_iter", |b| b.iter(|| sum_array_iter(&arr)));
   c.bench_function("sum_array_rayon", |b| b.iter(|| sum_array_rayon(&arr)));
   c.bench_function("sum_array_simd", |b| b.iter(|| sum_array_simd(&arr)));
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
