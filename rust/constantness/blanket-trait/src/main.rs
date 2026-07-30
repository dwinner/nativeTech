trait Blanket {}

impl<T> Blanket for T {}

#[derive(Debug)]
struct Buffer<T, const LEN: usize>
{
   buf: [T; LEN],
}

impl<T: Default + Copy, const LEN: usize> From<Vec<T>> for Buffer<T, LEN>
{
   fn from(vec: Vec<T>) -> Self
   {
      assert_eq!(LEN, vec.len());
      
      let mut ret = Self {
         buf: [T::default(); LEN],
      };
      ret.buf.copy_from_slice(&vec);
      
      ret
   }
}

fn main()
{
   const LEN: usize = 8;
   let group_of_seven = vec![
      "Canada",
      "France",
      "Germany",
      "Italy",
      "Japan",
      "United Kingdom",
      "United States",
      "European Union",
   ];
   let g7_buf: Buffer<&str, LEN> = Buffer::from(group_of_seven);
   dbg!(&g7_buf);
}
