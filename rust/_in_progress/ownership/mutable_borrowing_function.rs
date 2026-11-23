struct Account {
    balance: f64,
}

fn deposit(account: &mut Account, amount: f64) {
    account.balance += amount;
    println!("Deposited ${:.2}, new balance is ${:.2}", amount, account.balance);
}

fn main() {
    let mut my_account = Account { balance: 1000.0 };

    deposit(&mut my_account, 200.0); // Mutably borrowing the struct
    println!("Final balance is ${:.2}", my_account.balance); // The
    // original data is modified
}