mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Counter
{
   value: i32,
}

#[wasm_bindgen]
impl Counter
{
   // Конструктор
   #[wasm_bindgen(constructor)]
   pub fn new() -> Counter
   {
      Counter { value: 0 }
   }
   // Увеличение счётчика
   pub fn increment(&mut self)
   {
      self.value += 1;
   }
   // Уменьшение счётчика
   pub fn decrement(&mut self)
   {
      self.value -= 1;
   }
   // Получение текущего значения
   pub fn get_value(&self) -> i32
   {
      self.value
   }
}

#[wasm_bindgen]
pub fn log_message(msg: &str)
{
   web_sys::console::log_1(&msg.into());
}
