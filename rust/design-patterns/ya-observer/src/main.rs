use std::cell::RefCell;
use std::rc::Rc;

/// `Observer` trait
trait Observer
{
   fn update(&self, state: i32);
}

/// Subject
struct Subject
{
   observers: Vec<Rc<RefCell<dyn Observer>>>,
   state: i32,
}

impl Subject
{
   fn new() -> Self
   {
      Subject {
         observers: Vec::new(),
         state: 0,
      }
   }

   fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>)
   {
      self.observers.push(observer);
   }

   fn set_state(&mut self, state: i32)
   {
      self.state = state;
      self.notify();
   }

   fn notify(&self)
   {
      for observer in &self.observers
      {
         observer.borrow().update(self.state);
      }
   }
}

/// Concrete observer
struct ConsoleLogger;
impl Observer for ConsoleLogger
{
   fn update(&self, state: i32)
   {
      println!("State updated to: {}", state);
   }
}

fn main()
{
   let mut subject = Subject::new();
   let logger = Rc::new(RefCell::new(ConsoleLogger));
   subject.attach(logger);

   subject.set_state(42); // "State updated to: 42"
   subject.set_state(100); // "State updated to: 100"
}
