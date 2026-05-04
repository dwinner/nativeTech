use std::cell::RefCell;
use std::rc::Rc;

struct Node
{
   value: i32,
   children: Vec<Rc<RefCell<Node>>>,
}

fn main()
{
   let root = Rc::new(RefCell::new(Node {
      value: 1,
      children: Vec::new(),
   }));
   let child = Rc::new(RefCell::new(Node {
      value: 2,
      children: Vec::new(),
   }));
   root.borrow_mut().children.push(child.clone());
   println!("Root value: {}", root.borrow().value); // 1
   println!("Child value: {}", child.borrow().value); // 2
}
