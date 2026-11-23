// 08_multiple_producers.rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx_main, rx_main): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // Clone the sender to demonstrate multiple producers
    let tx_producer1 = tx_main.clone();

    // Producer thread 1
    let handle1 = thread::spawn(move || {
        tx_producer1.send("Message Alpha from Producer 1".to_string()).unwrap();
        thread::sleep(Duration::from_millis(150));
        tx_producer1.send("Message Beta from Producer 1".to_string()).unwrap();
        println!("Producer 1: All messages sent.");
        // tx_producer1 is dropped here when the thread ends
    });

    // Producer thread 2 (using the original tx_main, which was moved)
    let handle2 = thread::spawn(move || {
        tx_main.send("Message Gamma from Producer 2".to_string()).unwrap();
        thread::sleep(Duration::from_millis(80));
        tx_main.send("Message Delta from Producer 2".to_string()).unwrap();
        println!("Producer 2: All messages sent.");
        // tx_main is dropped here when the thread ends
    });

    println!("Main Thread (Consumer): Waiting for messages from producers...");

    /* Using rx_main as an iterator in a for loop is a very clean and idiomatic way 
       to receive all messages. However, it's important to understand how this loop behaves, 
       as it's different from iterating over a Vec. This loop will block if the channel 
       is temporarily empty and wait for the next message to arrive. It does not stop 
       just because the channel is empty. The loop will only terminate and allow the 
       program to continue to the next line when all Senders (tx and tx1 in our example) 
       have been dropped, which signals that no more messages will ever be sent.
    */
    for received_message_content in rx_main {
        println!("Main Thread (Consumer): Received: '{}'", received_message_content);
    }

    println!("Main Thread (Consumer): Channel disconnected, all producers have finished.");

    // Ensure both producer threads have completed their execution
    handle1.join().expect("Producer 1 thread panicked!");
    handle2.join().expect("Producer 2 thread panicked!");

    println!("\n--- try_recv() Demonstration ---");

    // Example demonstrating try_recv()
    let (tx_single_msg, rx_single_msg) = mpsc::channel::<i32>();

    // Attempt to receive when channel is empty
    match rx_single_msg.try_recv() {
        Ok(msg) => println!("try_recv (1): Unexpectedly got a message: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("try_recv (1): Channel is confirmed empty."),
        Err(mpsc::TryRecvError::Disconnected) => println!("try_recv (1): Channel is disconnected."),
    }

    // Send a message
    tx_single_msg.send(101).expect("Failed to send on single_msg channel");

    // Attempt to receive again
    match rx_single_msg.try_recv() {
        Ok(msg) => println!("try_recv (2): Got the message: {}", msg), // This will be executed
        Err(mpsc::TryRecvError::Empty) => println!("try_recv (2): Channel is still empty (unexpected)."),
        Err(mpsc::TryRecvError::Disconnected) => println!("try_recv (2): Channel is disconnected (unexpected)."),
    }

    // Drop the sender, then try_recv again
    drop(tx_single_msg);

    match rx_single_msg.try_recv() {
        Ok(msg) => println!("try_recv (3): Unexpectedly got a message after drop: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("try_recv (3): Channel is empty after drop (unexpected)."),
        Err(mpsc::TryRecvError::Disconnected) => println!("try_recv (3): Channel correctly reported as disconnected."), // This
    }
}