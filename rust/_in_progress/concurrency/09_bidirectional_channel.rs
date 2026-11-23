// 09_bidirectional_channels.rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Channel for the main thread to signal the worker to start
    let (start_tx, start_rx) = mpsc::channel::<()>(); // Using unit type for a pure signal
    
    // Channel for the worker to send its result back to the main thread
    let (result_tx, result_rx) = mpsc::channel::<String>();

    let worker_handle = thread::spawn(move || {
        println!("Worker Thread: Initialized and waiting for the green light...");
        
        // Block until a () signal is received on start_rx
        start_rx.recv().expect("Failed to receive start signal from main thread.");
        
        println!("Worker Thread: Green light received! Performing complex task...");
        thread::sleep(Duration::from_secs(1)); // Simulate some work
        
        let computation_result = "Task completed successfully by worker!".to_string();
        
        // Send the result back to the main thread
        result_tx.send(computation_result).expect("Failed to send result to main thread.");
        println!("Worker Thread: Result sent, finishing up.");
    });

    println!("Main Thread: Performing some setup before signaling worker...");
    thread::sleep(Duration::from_millis(500)); // Simulate setup work

    println!("Main Thread: Setup complete. Sending start signal to worker...");
    start_tx.send(()).expect("Failed to send start signal to worker."); // Send the () signal

    // Block and wait for the worker thread to send back its result
    println!("Main Thread: Waiting for result from worker...");
    let worker_output = result_rx.recv().expect("Failed to receive result from worker.");
    
    println!("Main Thread: Received from worker: '{}'", worker_output);

    // Ensure the worker thread has fully completed its execution
    worker_handle.join().expect("Worker thread panicked during execution!");
    println!("Main Thread: Worker thread has joined. Program exiting.");
}