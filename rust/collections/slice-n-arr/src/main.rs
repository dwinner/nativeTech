fn main()
{
   let numbers = [1, 2, 3, 4, 5];

   let sum = array_sum(&numbers);
   println!("Сумма: {}", sum); // 15

   let (min, max) = tuple_min_max(&numbers);
   println!("Минимум: {}, Максимум: {}", min, max); // 1, 5

   let evens = even_numbers(&numbers);
   println!("Чётные числа: {:?}", evens); // [2, 4]
}

fn array_sum(arr: &[i32; 5]) -> i32
{
   arr.iter().sum()
}

fn tuple_min_max(arr: &[i32; 5]) -> (i32, i32)
{
   let min = *arr.iter().min().unwrap();
   let max = *arr.iter().max().unwrap();

   (min, max)
}

fn even_numbers(arr: &[i32; 5]) -> Vec<i32>
{
   let mut vec = Vec::new();
   for &num in arr
   {
      if num % 2 == 0
      {
         vec.push(num);
      }
   }

   vec
}
