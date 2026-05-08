trait CalculationStrategy
{
   fn calculate(&self, a: i32, b: i32) -> i32;
}

struct AddStrategy;
impl CalculationStrategy for AddStrategy
{
   fn calculate(&self, a: i32, b: i32) -> i32
   {
      a + b
   }
}

struct MultiplyStrategy;
impl CalculationStrategy for MultiplyStrategy
{
   fn calculate(&self, a: i32, b: i32) -> i32
   {
      a * b
   }
}

struct Calculator
{
   strategy: Box<dyn CalculationStrategy>,
}

impl Calculator
{
   fn new(strategy: Box<dyn CalculationStrategy>) -> Self
   {
      Calculator { strategy }
   }

   fn set_strategy(&mut self, strategy: Box<dyn CalculationStrategy>)
   {
      self.strategy = strategy;
   }

   fn execute(&self, a: i32, b: i32) -> i32
   {
      self.strategy.calculate(a, b)
   }
}

fn main()
{
   let mut calc = Calculator::new(Box::new(AddStrategy));
   println!("Add: {}", calc.execute(3, 4)); // 7

   calc.set_strategy(Box::new(MultiplyStrategy));
   println!("Multiply: {}", calc.execute(3, 4)); // 12
}
