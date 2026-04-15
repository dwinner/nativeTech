enum TrafficLight
{
   Red,
   Yellow,
   Green,
}

fn main()
{
   let mut state = TrafficLight::Red;
   let mut timer = 0;

   loop
   {
      match state
      {
         TrafficLight::Red =>
         {
            println!("Red: STOP!");
            if timer >= 3
            {
               state = TrafficLight::Green;
               timer = 0;
            }
         }
         TrafficLight::Yellow =>
         {
            println!("Yellow: GET READY!");
            if timer >= 2
            {
               state = TrafficLight::Red;
               timer = 0;
            }
         }
         TrafficLight::Green =>
         {
            println!("Green: GO!");
            if timer >= 5
            {
               state = TrafficLight::Yellow;
               timer = 0;
            }
         }
      }

      timer += 1;
      println!("Timer: {}", timer);

      if timer > 10
      {
         break;
      }
   }
}
