use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

fn write_numbers_to_file() -> (u32, i8, f64) {
   // slices implement Read and Write, and can thus act as mock files
   let mut in_vec = vec![];

   let one: u32 = 1;
   let two: i8 = 2;
   let three: f64 = 3.0;

   in_vec.write_u32::<LittleEndian>(one).unwrap();
   println!("{:?}", &in_vec);

   // single byte methods don't take an endianness parameter
   in_vec.write_i8(two).unwrap();
   println!("{:?}", &in_vec);

   in_vec.write_f64::<LittleEndian>(three).unwrap();
   println!("{:?}", &in_vec);

   (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
   let mut cursor = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
   let one_ = cursor.read_u32::<LittleEndian>().unwrap();
   let two_ = cursor.read_i8().unwrap();
   let three_ = cursor.read_f64::<LittleEndian>().unwrap();

   (one_, two_, three_)
}

fn main() {
   write_numbers_to_file();
   read_numbers_from_file();
}
