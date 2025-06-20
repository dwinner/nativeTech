fn main()
{
   let req_size: usize = 18_446_744_073_709_550_000;
   _ = try_build_a_vec(req_size);
}

fn try_build_a_vec(required_size: usize) -> Result<Vec<u8>, String>
{
   let mut vector = Vec::new();
   vector
      .try_reserve(required_size)
      .map_err(|_e| format!("Failed to allocate {} items!", required_size))?;

   // Ok, at this point we know it's safe enough
   vector.push(1);
   vector.push(1);
   vector.push(1);
   vector.push(1);

   Ok(vector)
}
