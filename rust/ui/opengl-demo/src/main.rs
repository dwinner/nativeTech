use glow::HasContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

fn main()
{
   let event_loop = EventLoop::new().unwrap();
   let window = winit::window::WindowBuilder::new()
      .with_title("Моё приложение")
      .build(&event_loop)
      .unwrap();
   let gl_context = unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) };
   let gl = &gl_context;

   event_loop
      .run(move |event, _, control_flow| {
         *control_flow = ControlFlow::Wait;
         match event
         {
            Event::WindowEvent {
               event: WindowEvent::CloseRequested,
               ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) =>
            {
               unsafe {
                  gl.clear_color(0.0, 0.5, 0.0, 1.0); // Зелёный фон
                  gl.clear(glow::COLOR_BUFFER_BIT);
               }
               window.request_redraw();
            }
            _ =>
            {}
         }
      })
      .unwrap();
}
