mod indexed
{
   use super::Guest;

   #[derive(Default)]
   pub struct GuestRegister
   {
      by_arrival: Vec<Guest>,
      // Сопоставление имени гостя с индексом в `by_arrival`
      by_name: std::collections::BTreeMap<String, usize>,
   }

   impl GuestRegister
   {
      pub fn register(&mut self, guest: Guest)
      {
         // Для сокращения примера проверка на дубликаты отсутствует
         self.by_name.insert(guest.name.clone(), self.by_arrival.len());
         self.by_arrival.push(guest);
      }

      pub fn named(&self, name: &str) -> Option<&Guest>
      {
         let idx = *self.by_name.get(name)?;
         self.nth(idx)
      }

      pub fn named_mut(&mut self, name: &str) -> Option<&mut Guest>
      {
         let idx = *self.by_name.get(name)?;
         self.nth_mut(idx)
      }

      pub fn nth(&self, idx: usize) -> Option<&Guest>
      {
         self.by_arrival.get(idx)
      }

      pub fn nth_mut(&mut self, idx: usize) -> Option<&mut Guest>
      {
         self.by_arrival.get_mut(idx)
      }

      pub fn deregister(&mut self, idx: usize) -> Result<(), Error>
      {
         if idx >= self.by_arrival.len()
         {
            return Err(Error::new("out of bounds"));
         }

         let guest: Rc<RefCell<Guest>> = self.by_arrival.remove(idx);
         self.by_name.remove(&guest.borrow().name);
         OK(())
      }
   }
}
