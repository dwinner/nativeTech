fn main()
{
   let mut num = 5;

   // Создаем сырые указатели из ссылок
   let r1 = &num as *const i32; // Неизменяемый указатель
   let r2 = &mut num as *mut i32; // Изменяемый указатель

   unsafe {
      println!("r1: {}", *r1); // Читаем через *const
      *r2 = 10; // Пишем через *mut
      println!("r2: {}", *r2);
   }

   println!("num: {}", num); // Выведет: num: 10

   // UB!
   let ptr: *mut i32 = std::ptr::null_mut();
   unsafe {
      *ptr = 42; // UB! Разыменование нулевого указателя No newline at end of file
   }
}
