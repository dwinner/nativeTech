#![feature(generic_const_exprs)]

fn main()
{
   const FACT_5_GENERIC: u32 = Factorial::<5>::VALUE; // 120
   println!("{}", FACT_5_GENERIC);
}

struct Factorial<const N: u32>;

impl<const N: u32> Factorial<N>
{
   const VALUE: u32 = if N == 0 { 1 } else { N * Factorial::<{ N - 1 }>::VALUE };
}
