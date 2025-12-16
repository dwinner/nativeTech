use crate::Direction::Bottom;

fn main()
{
   let direction: Direction = Direction::Bottom;
   let thruster_force: PoundForceSeconds = thruster_impulse(direction);
   let new_direction = update_trajectory(thruster_force.into());
   println!("{:?}", new_direction);
}

pub enum Direction
{
   Top,
   Bottom,
}

pub struct PoundForceSeconds(pub f64);

pub fn thruster_impulse(direction: Direction) -> PoundForceSeconds
{
   let pfs: f64 = match direction
   {
      Direction::Top => 42.0,
      Bottom => -42.0,
   };

   PoundForceSeconds(pfs)
}

pub struct NewtonSeconds(pub f64);

pub fn update_trajectory(_force: NewtonSeconds) {}

impl From<PoundForceSeconds> for NewtonSeconds
{
   fn from(value: PoundForceSeconds) -> Self
   {
      NewtonSeconds(4.448222 * value.0)
   }
}
