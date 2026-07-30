#[derive(Debug)]
struct Buffer<T, const LEN: usize>
{
   buf: [T; LEN],
}

impl<T, const LEN: usize> From<[T; LEN]> for Buffer<T, LEN>
{
   fn from(buf: [T; LEN]) -> Self
   {
      Buffer { buf }
   }
}

fn main()
{
   let buf = Buffer::from([0, 1, 2, 3]);
   dbg!(&buf);
}
