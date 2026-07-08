#![feature(portable_simd)]

use std::simd::Simd;

fn main()
{
   let a = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]];
   let b = [[17, 18, 19, 20], [21, 22, 23, 24], [25, 26, 27, 28], [29, 30, 31, 32]];
   let result = matrix_multiply_simd(&a, &b);
   for row in result.iter()
   {
      println!("{:?}", row);
   }
}

fn matrix_multiply_simd(a: &[[i32; 4]; 4], b: &[[i32; 4]; 4]) -> [[i32; 4]; 4]
{
   let mut result = [[0; 4]; 4];
   for i in 0..4
   {
      for j in 0..4
      {
         let mut sum = Simd::<i32, 4>::splat(0);
         for k in 0..4
         {
            let a_vec = Simd::from_array([a[i][k], a[i][k], a[i][k], a[i][k]]); // Вектор из a[i][k]
            let b_vec = Simd::from_array([b[k][j], b[k][j], b[k][j], b[k][j]]); // Вектор из b[k][j]
            sum += a_vec * b_vec; // Векторное умножение и накопление
         }

         result[i][j] = sum.as_array()[0]; // Все элементы sum одинаковы
      }
   }

   result
}
