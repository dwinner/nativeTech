fn main()
{
   let amount_to_add = 2;
   let add_n = |val: u32| -> u32 { val + amount_to_add };

   let mut data = vec![1, 2, 3];
   modify_all(&mut data, add_n);

   assert_eq!(data, vec![3, 4, 5]);
}

fn modify_all<F>(data: &mut [u32], mut mutator: F)
where
   F: FnMut(u32) -> u32,
{
   for val in data
   {
      *val = mutator(*val);
   }
}
