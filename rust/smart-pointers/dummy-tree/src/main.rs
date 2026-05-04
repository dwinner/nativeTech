use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode
{
   value: i32,
   left: Option<Rc<RefCell<TreeNode>>>,
   right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode
{
   fn new(value: i32) -> Self
   {
      TreeNode {
         value,
         left: None,
         right: None,
      }
   }

   fn insert(&mut self, value: i32)
   {
      if value < self.value
      {
         match &self.left
         {
            Some(left) => left.borrow_mut().insert(value),
            None => self.left = Some(Rc::new(RefCell::new(TreeNode::new(value)))),
         }
      }
      else
      {
         match &self.right
         {
            Some(right) => right.borrow_mut().insert(value),
            None => self.right = Some(Rc::new(RefCell::new(TreeNode::new(value)))),
         }
      }
   }

   fn inorder(&self)
   {
      if let Some(ref left) = self.left
      {
         left.borrow().inorder();
      }

      println!("{}", self.value);

      if let Some(ref right) = self.right
      {
         right.borrow().inorder();
      }
   }
}

fn main()
{
   let root = Rc::new(RefCell::new(TreeNode::new(5)));
   root.borrow_mut().insert(3);
   root.borrow_mut().insert(7);
   root.borrow_mut().insert(1);
   root.borrow_mut().insert(9);
   println!("Inorder traversal:");
   root.borrow().inorder(); // 1, 3, 5, 7, 9
}
