use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

struct ListItem<T>
{
   data: ItemData<T>,
   next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T>
{
   fn new(data: T) -> Self
   {
      Self {
         data: Rc::new(RefCell::new(data)),
         next: None,
      }
   }
}

struct LinkedList<T>
{
   head: ListItemPtr<T>,
}

impl<T> LinkedList<T>
{
   fn new(data: T) -> Self
   {
      Self {
         head: Rc::new(RefCell::new(ListItem::new(data))),
      }
   }

   fn append(&mut self, data: T)
   {
      let mut next = self.head.clone();
      while next.as_ref().borrow().next.is_some()
      {
         let list_item_ptr = next.as_ref().borrow().next.as_ref().unwrap().clone();
         next = list_item_ptr;
      }

      next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(data))));
   }

   fn iter(&'_ self) -> Iter<'_, T>
   {
      Iter {
         next: Some(self.head.clone()),
         data: None,
         phantom: PhantomData,
      }
   }

   fn iter_mut(&'_ mut self) -> IterMut<'_, T>
   {
      IterMut {
         next: Some(self.head.clone()),
         data: None,
         phantom: PhantomData,
      }
   }

   fn into_iter(self) -> IntoIter<T>
   {
      IntoIter {
         next: Some(self.head.clone()),
      }
   }
}

struct Iter<'a, T>
{
   next: Option<ListItemPtr<T>>,
   data: Option<ItemData<T>>,
   phantom: PhantomData<&'a T>,
}

struct IterMut<'a, T>
{
   next: Option<ListItemPtr<T>>,
   data: Option<ItemData<T>>,
   phantom: PhantomData<&'a T>,
}

struct IntoIter<T>
{
   next: Option<ListItemPtr<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
{
   type Item = &'a T;

   fn next(&mut self) -> Option<Self::Item>
   {
      match self.next.clone()
      {
         Some(ptr) =>
         {
            self.next.clone_from(&ptr.as_ref().borrow().next);
            self.data = Some(ptr.as_ref().borrow().data.clone());
            unsafe { Some(&*self.data.as_ref().unwrap().as_ptr()) }
         }
         None => None,
      }
   }
}

impl<'a, T> Iterator for IterMut<'a, T>
{
   type Item = &'a mut T;

   fn next(&mut self) -> Option<Self::Item>
   {
      match self.next.clone()
      {
         Some(ptr) =>
         {
            self.next.clone_from(&ptr.as_ref().borrow().next);
            self.data = Some(ptr.as_ref().borrow().data.clone());
            unsafe { Some(&mut *self.data.as_ref().unwrap().as_ptr()) }
         }
         None => None,
      }
   }
}

impl<T> Iterator for IntoIter<T>
{
   type Item = T;

   fn next(&mut self) -> Option<Self::Item>
   {
      match self.next.clone()
      {
         Some(ptr) =>
         {
            self.next.clone_from(&ptr.as_ref().borrow().next);
            let list_item = Rc::try_unwrap(ptr).map(|refcell| refcell.into_inner());
            match list_item
            {
               Ok(ok_list_item) => Rc::try_unwrap(ok_list_item.data)
                  .map(|refcell| refcell.into_inner())
                  .ok(),
               Err(_) => None,
            }
         }
         None => None,
      }
   }
}

impl<'a, T> IntoIterator for &'a LinkedList<T>
{
   type Item = &'a T;
   type IntoIter = Iter<'a, T>;

   fn into_iter(self) -> Self::IntoIter
   {
      self.iter()
   }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T>
{
   type Item = &'a mut T;
   type IntoIter = IterMut<'a, T>;

   fn into_iter(self) -> Self::IntoIter
   {
      self.iter_mut()
   }
}

impl<T> IntoIterator for LinkedList<T>
{
   type Item = T;
   type IntoIter = IntoIter<T>;

   fn into_iter(self) -> Self::IntoIter
   {
      self.into_iter()
   }
}

fn main()
{
   let mut dinosaurs = LinkedList::new("Tyrannosaurus Rex");
   dinosaurs.append("Triceratops");
   dinosaurs.append("Velociraptor");
   dinosaurs.append("Stegosaurus");
   dinosaurs.append("Spinosaurus");

   for data in &dinosaurs
   {
      println!("with for loop: data={}", data);
   }

   dinosaurs.iter().for_each(|data| println!("with iter(): data={}", data));

   dinosaurs
      .iter_mut()
      .for_each(|data| println!("with iter_mut(): data={}", data));

   dinosaurs
      .into_iter()
      .for_each(|data| println!("with into_iter(): data={}", data));
}
