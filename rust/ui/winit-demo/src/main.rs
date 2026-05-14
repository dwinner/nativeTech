use softbuffer::GraphicsContext;
use winit::{
   event::{Event, WindowEvent},
   event_loop::{ControlFlow, EventLoop},
   window::WindowBuilder,
};

fn main()
{
   let event_loop = EventLoop::new().unwrap();
   let window = WindowBuilder::new()
      .with_title("Простое окно на Rust")
      .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
      .build(&event_loop)
      .unwrap();

   let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

   event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;

      match event
      {
         Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
         } => *control_flow = ControlFlow::Exit,
         Event::RedrawRequested(_) =>
         {
            let (width, height) = window.inner_size().into();
            let buffer = vec![0xFF00FF00; (width * height) as usize]; // Зелёный фон
            graphics_context.set_buffer(&buffer, width as u16, height as u16);
         }
         _ => (),
      }
   });
}
