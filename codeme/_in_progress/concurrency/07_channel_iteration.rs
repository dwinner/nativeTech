// 07_channel_iteration.rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // Spawn a thread that will send messages
    let sender_thread_handle = thread::spawn(move || {
        let messages_to_send = vec![
            String::from("Greetings"),
            String::from("from"),
            String::from("the producer"),
            String::from("thread!"),
        ];

        for msg_content in messages_to_send {
            println!("Sender Thread: Preparing to send '{}'", msg_content);
            // Send the message. send() takes ownership of msg_content.
            if tx.send(msg_content).is_err() {
                // This error would occur if the receiver (rx) was dropped.
                eprintln!("Sender Thread: Receiver has disconnected, unable to send further messages.");
                break; // Exit the loop if we can't send
            }
            println!("Sender Thread: Message sent successfully.");
            thread::sleep(Duration::from_millis(200)); // Simulate some work
        }
        println!("Sender Thread: All messages dispatched or receiver gone.");
    });

    // Main thread will now try to receive.
    // The receiver (rx) is still in scope here.
    println!("Main Thread: Waiting for messages from sender thread...");
    
    // rx can be used as an iterator. It blocks waiting for messages and stops
    // when the channel is closed (i.e., when tx goes out of scope).
    for received_message in rx { 
        println!("Main Thread: Received: '{}'", received_message);
    }

    println!("Main Thread: Channel disconnected (all senders dropped).");

    // Wait for the sender thread to finish its execution completely
    sender_thread_handle.join().expect("Sender thread panicked!");
    println!("Main Thread: Sender thread has joined.");
}