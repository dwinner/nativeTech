fn main() {
    let data = String::from("Important data");

    process_data(data);

    // This line would cause a compile-time error
    // println!("{}", data);
}

fn process_data(input: String) {
    println!("Processing: {}", input);
}