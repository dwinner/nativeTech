fn main() {
    let original_owner = String::from("Rustacean");
    let new_owner = original_owner; // Ownership moves to new_owner

    // println!("{}", original_owner); // Error: original_owner is invalid
    println!("{}", new_owner); // new_owner is now the owner
}