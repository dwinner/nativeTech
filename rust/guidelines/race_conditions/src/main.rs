use std::sync::Mutex;

fn main()
{
   {
      let account = std::sync::Arc::new(BankAccount::new());
      account.deposit(1_000);

      let account2 = account.clone();
      let _taker = std::thread::spawn(move || take_out(&account2));

      let account3 = account.clone();
      let _payer = std::thread::spawn(move || pay_in(&account3));
   }
}

pub fn pay_in(account: &BankAccount)
{
   loop
   {
      if account.balance() < 200
      {
         println!("[A] Running low, deposit 400");
         account.deposit(400);
      }

      std::thread::sleep(std::time::Duration::from_millis(5));
   }
}

pub fn take_out(account: &BankAccount)
{
   loop
   {
      if account.withdraw(100)
      {
         println!("[B] Withdrew 100, balance now {}", account.balance());
      }
      else
      {
         println!("[B] Failed to withdraw 100");
      }

      std::thread::sleep(std::time::Duration::from_millis(20));
   }
}

pub struct BankAccount
{
   balance: std::sync::Mutex<i64>,
}

impl BankAccount
{
   pub fn new() -> Self
   {
      BankAccount { balance: Mutex::new(0) }
   }

   pub fn balance(&self) -> i64
   {
      let balance = *self.balance.lock().unwrap();
      if balance < 0
      {
         panic!("** Oh no, gone overdrawn: {}", balance);
      }

      balance
   }

   // NOTE: no need `&mut self` anymore
   pub fn deposit(&self, amount: i64)
   {
      *self.balance.lock().unwrap() += amount
   }

   pub fn withdraw(&self, amount: i64) -> bool
   {
      let mut balance = self.balance.lock().unwrap();
      if (*balance) < amount
      {
         return false;
      }

      *balance -= amount;
      true
   }
}
