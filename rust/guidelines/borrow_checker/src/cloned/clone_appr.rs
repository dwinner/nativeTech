mod cloned
{
   use super::Guest;

   pub struct GuestRegister
   {
      by_arrival: Vec<Guest>,
      by_name: std::collections::BTreeMap<String, Guest>,
   }

   impl GuestRegister
   {
      pub fn register(&mut self, guest: Guest)
      {
         // requires that `Guest` was `Clone`
         self.by_arrival.push(guest.clone());

         // missing duplicates checking
         self.by_name.insert(guest.name.clone(), guest);
      }

      pub fn named(&self, name: &str) -> Option<&Guest>
      {
         self.by_name.get(name)
      }

      pub fn nth(&self, idx: usize) -> Option<&Guest>
      {
         self.by_arrival.get(idx)
      }
   }
}
