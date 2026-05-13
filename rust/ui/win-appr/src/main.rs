//rust
use windows::{
   core::*, Win32::Foundation::*, Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()>
{
   unsafe {
      let instance = GetModuleHandleA(None)?;
      let wc = WNDCLASSA {
         style: CS_HREDRAW | CS_VREDRAW,
         lpfnWndProc: Some(wnd_proc),
         hInstance: instance,
         lpszClassName: s!("RustWindowClass"),
         ..Default::default()
      };
      RegisterClassA(&wc)?;

      let hwnd = CreateWindowExA(
         WINDOW_EX_STYLE::default(),
         s!("RustWindowClass"),
         s!("Окно с кнопкой"),
         WS_OVERLAPPEDWINDOW | WS_VISIBLE,
         CW_USEDEFAULT,
         CW_USEDEFAULT,
         800,
         600,
         None,
         None,
         instance,
         None,
      );

      CreateWindowExA(
         WINDOW_EX_STYLE::default(),
         s!("BUTTON"),
         s!("Нажми меня"),
         WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
         50,
         50,
         100,
         30,
         hwnd,
         HMENU(1),
         instance,
         None,
      );

      let mut msg = MSG::default();
      while GetMessageA(&mut msg, None, 0, 0).into()
      {
         TranslateMessage(&msg);
         DispatchMessageA(&msg);
      }
      Ok(())
   }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
   unsafe {
      match msg
      {
         WM_COMMAND =>
         {
            if wparam.0 == 1
            {
               println!("Кнопка нажата!");
            }
            LRESULT(0)
         }
         WM_DESTROY =>
         {
            PostQuitMessage(0);
            LRESULT(0)
         }
         _ => DefWindowProcA(hwnd, msg, wparam, lparam),
      }
   }
}
