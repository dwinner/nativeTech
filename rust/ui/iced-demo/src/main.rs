use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

#[derive(Default)]
struct CounterApp
{
   counter: i32,
}

#[derive(Debug, Clone)]
enum Message
{
   Increment,
}

impl Sandbox for CounterApp
{
   type Message = Message;

   fn new() -> Self
   {
      Self::default()
   }

   fn title(&self) -> String
   {
      "Счётчик".into()
   }

   fn update(&mut self, message: Message)
   {
      match message
      {
         Message::Increment => self.counter += 1,
      }
   }

   fn view(&self) -> Element
   {
      column![
         text(format!("Счётчик: {}", self.counter)),
         button("Нажми").on_press(Message::Increment),
      ]
      .into()
   }
}

fn main() -> iced::Result
{
   CounterApp::run(Settings::default())
}
