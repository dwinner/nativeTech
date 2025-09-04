fn main()
{
   // iterating over errors
   let strings = vec!["tofu", "93", "18"];
   let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
   println!("Results: {:?}", numbers);

   // ignore failed items
   let strings = vec!["tofu", "93", "18"];
   let numbers: Vec<_> = strings.into_iter().filter_map(|s| s.parse::<i32>().ok()).collect();
   println!("Results: {:?}", numbers);

   // collect failed items
   let strings = vec!["42", "tofu", "93", "999", "18"];
   let mut errors = vec![];
   let numbers: Vec<_> = strings
      .into_iter()
      .map(|s| s.parse::<u8>())
      .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
      .collect();
   println!("Numbers: {:?}", numbers);
   println!("Errors: {:?}", errors);

   // fail entire operation
   let strings = vec!["tofu", "93", "18"];
   let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
   println!("Results: {:?}", numbers);

   // collect all valid values and failures
   let strings = vec!["tofu", "93", "18"];
   let (numbers, errors): (Vec<_>, Vec<_>) = strings.into_iter().map(|s| s.parse::<i32>()).partition(Result::is_ok);
   let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
   let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
   println!("Numbers: {:?}", numbers);
   println!("Errors: {:?}", errors);
}
