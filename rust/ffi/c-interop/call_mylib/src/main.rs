#[link(name = "libmylib", kind = "raw-dylib")]
unsafe extern "C" {
   fn add(a: i32, b: i32) -> i32;
}

fn main() {
   let result = unsafe { add(3, 4) };
   println!("Result: {}", result);
}
