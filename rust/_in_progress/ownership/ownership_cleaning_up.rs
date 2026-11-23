fn main() {
    {
        let temp_owner = String::from("Temporary");
        println!("{}", temp_owner); // temp_owner is valid here
    } // temp_owner is dropped here, and the memory is freed
    // println!("{}", temp_owner); // Error: temp_owner is invalid
}