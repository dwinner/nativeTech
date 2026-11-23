// 06_channel_basics.rs
use std::sync::mpsc; // Import the mpsc module

fn main() {
    // Create a new channel. The type of data sent will be i32.
    // mpsc stands for "Multiple Producer, Single Consumer".
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // tx is the Sender (transmitter)
    // rx is the Receiver
    
    // Note: Sender and Receiver implement Debug, so we can print them.
    println!("Channel created successfully! Sender: {:?}, Receiver: {:?}", tx, rx);

    // We'll see how to use tx and rx next.
}