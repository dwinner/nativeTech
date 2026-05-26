use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// Предполагается, что sum_vector доступна из корневого модуля
use sum_vec_bench::sum_vector;

fn bench_sum_vector(criteria: &mut Criterion)
{
   let vec = vec![1; 10_000]; // Вектор из 10_000 единиц
   criteria.bench_function("sum_vector", |bencher| {
      bencher.iter(|| sum_vector(&vec));
   });
}

fn bench_sum_vector_blackbox(criteria: &mut Criterion)
{
   let vec = vec![1; 10_000];
   criteria.bench_function("sum_vector", |bencher| {
      bencher.iter(|| std::hint::black_box(sum_vector(&vec)));
   });
}

fn bench_sum_vector_param(criteria: &mut Criterion)
{
   let mut group = criteria.benchmark_group("sum_vector");
   for size in [1_000, 10_000, 100_000].iter()
   {
      let vec = vec![1; *size];
      group.bench_with_input(BenchmarkId::new("sum", size), size, |bencher, _| {
         bencher.iter(|| std::hint::black_box(sum_vector(&vec)));
      });
   }
   group.finish();
}

criterion_group!(
   benches,
   bench_sum_vector,
   bench_sum_vector_blackbox,
   bench_sum_vector_param
);
criterion_main!(benches);
