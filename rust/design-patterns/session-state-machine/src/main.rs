use crate::fsm::Session;

mod fsm;

fn main()
{
   new_session();
   transitions();
}

fn new_session()
{
   let session = Session::new();
   println!("{:?}", session);
}

fn transitions()
{
   let session = Session::new();
   println!("{:?}", session);
   if let Ok(mut session) = session.authenticate("username", "password")
   {
      session.update_property("some.preference.bool", "true");
      println!("{:?}", session);
      let session = session.logout();
      println!("{:?}", session);
   }
}
