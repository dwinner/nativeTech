fn main()
{
   let values: Vec<u64> = vec![1, 1, 2, 3, 5];

   let (sum_squares, even_count) = explicit_cycle_sample(&values);
   println!("Sum of squares: {:?}", sum_squares);
   println!("Count of even: {:?}", even_count);

   let sum = functional_sample(&values);
   println!("Sum of squares: {:?}", sum);

   use_thing_iter();

   //undesired_behavior();

   fix_undesired_behavior();
}

/*fn undesired_behavior(){
   let inputs:Vec<i64>= vec![0,1,2,3,4,512];
   let result: Vec<u8>=inputs.into_iter().map(|v| <u8>::try_from(v).unwrap()).collect();
   println!("{result:?}");
}*/

fn fix_undesired_behavior()
{
   // using turbo fish
   let inputs: Vec<i64> = vec![0, 1, 2, 3, 4, 512];
   let result = inputs.into_iter().map(|val| <u8>::try_from(val)).collect::<Result<Vec<_>, _>>()/*?*/;
   println!("{result:?}");
}

fn explicit_cycle_sample(values: &Vec<u64>) -> (u64, u32)
{
   let mut even_sum_squares = 0;
   let mut even_count = 0;
   for i in 0..values.len()
   {
      if values[i] % 2 != 0
      {
         continue;
      }

      even_sum_squares += values[i] * values[i];
      even_count += 1;
      if even_count == 5
      {
         break;
      }
   }

   (even_sum_squares, even_count)
}

fn functional_sample(values: &Vec<u64>) -> u64
{
   let even_sum_squares: u64 = values.iter().filter(|x| *x % 2 == 0).take(5).map(|x| x * x).sum();
   even_sum_squares
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Thing(u64);

fn use_thing_iter()
{
   let collection = vec![Thing(0), Thing(1), Thing(2), Thing(3)];
   for item in &collection
   {
      println!("{}", item.0);
   }

   println!("collection still around {collection:?}");

   let result: u64 = collection.iter().map(|thing| thing.0).sum();
   println!("{result:?}");
}
