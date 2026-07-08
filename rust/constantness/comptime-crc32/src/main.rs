fn main()
{
   const DATA: &[u8] = b"Hello, embedded!";
   const CRC: u32 = crc32(DATA);
   println!("CRC is {}", CRC);

   const FACT_5: u32 = factorial(5); // 120, вычислено на этапе компиляции
   println!("Pre-computed factorial of 5 is {}", FACT_5);
}

const fn crc32(data: &[u8]) -> u32
{
   let mut crc = 0xFFFFFFFF;
   let mut i = 0;
   while i < data.len()
   {
      crc ^= data[i] as u32;
      let mut j = 0;
      while j < 8
      {
         if crc & 1 != 0
         {
            crc = (crc >> 1) ^ 0xEDB88320;
         }
         else
         {
            crc >>= 1;
         }

         j += 1;
      }

      i += 1;
   }

   crc ^ 0xFFFFFFFF
}

const fn factorial(n: u32) -> u32
{
   if n == 0 { 1 } else { n * factorial(n - 1) }
}
