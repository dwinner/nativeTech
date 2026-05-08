fn sort<T: PartialOrd>(mut items: Vec<T>) -> Vec<T>
{
   items.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
   items
}

fn main()
{
   let numbers = vec![3, 1, 4, 1, 5];
   let sorted_numbers = sort(numbers);
   println!("{:?}", sorted_numbers); // [1, 1, 3, 4, 5]

   let strings = vec!["rust", "is", "awesome"];
   let sorted_strings = sort(strings);
   println!("{:?}", sorted_strings); // ["awesome", "is", "rust"]
}
