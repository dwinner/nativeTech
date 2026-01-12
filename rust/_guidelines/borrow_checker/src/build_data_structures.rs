#[derive(Clone, Debug)]
pub struct Guest
{
   name: String,
   address: String,
}

/// Local error type
#[derive(Clone, Debug)]
pub struct Error(String);

/// Guest registry, in arrival order
#[derive(Default, Debug)]
pub struct GuestRegister(Vec<Guest>);

impl GuestRegister
{
   pub fn register(&mut self, guest: Guest)
   {
      self.0.push(guest)
   }

   pub fn nth(&self, idx: usize) -> Option<&Guest>
   {
      self.0.get(idx)
   }
}
