fn filter_numbers<T>(numbers: Vec<T>, predicate: impl Fn(&T) -> bool) -> Vec<T>
where
   T: Clone,
{
   let mut result = Vec::new();
   for item in numbers.iter()
   {
      if predicate(item)
      {
         result.push(item.clone());
      }
   }

   result
}

fn main()
{
   let numbers = vec![1, 2, 3, 4, 5, 6];

   // Фильтрация чётных чисел
   let is_even = |x: &i32| *x % 2 == 0;
   let even_numbers = filter_numbers(numbers.clone(), is_even);
   println!("Чётные числа: {:?}", even_numbers); // Вывод: [2, 4, 6]

   // Фильтрация чисел больше 3
   let greater_than_three = |x: &i32| *x > 3;
   let big_numbers = filter_numbers(numbers, greater_than_three);
   println!("Числа > 3: {:?}", big_numbers); // Вывод: [4, 5, 6]
}
