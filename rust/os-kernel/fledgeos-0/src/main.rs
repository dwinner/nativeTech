#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> !
{
   intrinsics::abort();
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
   let framebuffer = 0xb8000 as *mut u8;

   unsafe {
      framebuffer.offset(1).write_volatile(0x30);
   }

   loop
   {}
}
