fn main()
{
   let dizzy = Details {
      given_name: "Dizzy".to_owned(),
      family_name: "Mixer".to_owned(),
      ..Default::default()
   };

   /*let also_bob = DetailsBuilder::new("Robert", "Builder")
   .middle_name("the")
   .preferred_name("Bob")
   .build();*/

   let mut builder = DetailsBuilder::new("Robert", "Builder");

   /* let builder= */
   builder.preferred_name("Bob");
   let bob = builder.build();

   println!("{:?}", dizzy);
   println!("{:?}", bob);
}

#[derive(Debug, Clone)]
pub struct PhoneNumberE164(pub String);

#[derive(Debug, Default)]
pub struct Details
{
   pub given_name: String,
   pub preferred_name: Option<String>,
   pub middle_name: Option<String>,
   pub family_name: String,
   pub mobile_phone: Option<PhoneNumberE164>,
}

pub struct DetailsBuilder(Details);

impl DetailsBuilder
{
   pub fn new(given_name: &str, family_name: &str) -> Self
   {
      DetailsBuilder(Details {
         given_name: given_name.to_owned(),
         preferred_name: None,
         middle_name: None,
         family_name: family_name.to_owned(),
         mobile_phone: None,
      })
   }

   pub fn preferred_name(&mut self, preferred_name: &str) -> &mut Self
   {
      self.0.preferred_name = Some(preferred_name.to_owned());
      self
   }

   pub fn middle_name(&mut self, middle_name: &str) -> &mut Self
   {
      self.0.middle_name = Some(middle_name.to_owned());
      self
   }

   pub fn build(self) -> Details
   {
      self.0
   }
}
