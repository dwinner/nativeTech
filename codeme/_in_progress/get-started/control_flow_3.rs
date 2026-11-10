fn main() {
    // If expression
    let temperature = 25;

    if temperature > 30 {
        println!("It's too hot! Better get a gelato! 🍦");
    } else if temperature < 10 {
        println!("It's freezing! Time for a warm espresso. ☕");
    } else {
        println!("Perfect weather for a walk in Rome! 🏛️");
    }

    // Loop with a countdown for espresso shots
    let mut shots = 0;

    loop {
        println!("Espresso shot #{}", shots + 1);
        shots += 1;
        if shots == 3 {
            println!("Enough caffeine for now! ☕🫸");
            break;
        }
    }

    // While Loop with an Italian road trip countdown
    let mut kilometers = 5;

    while kilometers > 0 {
        println!("{} km to the Amalfi Coast! 🗺️", kilometers);
        kilometers -= 1;
    }
    println!("We've arrived! Time for some pasta! 🍝");

    // For Loop with an exclusive range for pizza slices
    // for slice in 1..5 { // 1 to 4
    //     println!("Eating slice #{} of pizza 🍕", slice);
    // }
}