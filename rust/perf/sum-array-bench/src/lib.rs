#![feature(portable_simd)]

use rayon::prelude::*;
use std::simd::num::SimdFloat;
use std::simd::Simd;

pub fn sum_array(arr: &[f32]) -> f32
{
   let mut sum = 0.0;
   for &value in arr
   {
      sum += value;
   }
   sum
}

pub fn sum_array_iter(arr: &[f32]) -> f32
{
   arr.iter().sum()
}

pub fn sum_array_rayon(arr: &[f32]) -> f32
{
   arr.par_iter().sum()
}

pub fn sum_array_simd(arr: &[f32]) -> f32
{
   const LANES: usize = 4; // Размер SIMD-вектора, например, для SSE2
   let mut sum = Simd::<f32, LANES>::splat(0.0);
   let mut i = 0;

   // Обработка массива по частям размером LANES
   while i + LANES <= arr.len()
   {
      let chunk = Simd::from_slice(&arr[i..i + LANES]);
      sum += chunk;
      i += LANES;
   }

   // Сведение вектора в скалярную сумму
   let mut total = sum.reduce_sum();

   // Обработка остатка
   for &value in &arr[i..]
   {
      total += value;
   }

   total
}
