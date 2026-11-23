// 05_interior_mutability.rs

use std::cell::RefCell;
use std::rc::Rc;

// A logger that uses interior mutability to track its state.
// We use RefCell because we will share this Logger via Rc.
// Rc only allows immutable borrows (&T), so we need RefCell to mutate the data inside.
struct MessageLogger {
    message_count: RefCell<usize>,
    history: RefCell<Vec<String>>,
}

impl MessageLogger {
    fn new() -> Self {
        MessageLogger {
            message_count: RefCell::new(0),
            history: RefCell::new(Vec::new()),
        }
    }

    // This method takes &self (immutable reference).
    // From the outside, it looks like this method doesn't change the object.
    // However, internally, it uses RefCell to modify the state.
    fn log(&self, message: &str) {
        // borrow_mut() checks borrowing rules at RUNTIME.
        // If we try to borrow_mut while another borrow is active, the program will panic.
        let mut count = self.message_count.borrow_mut();
        *count += 1;

        self.history
            .borrow_mut()
            .push(format!("#{}: {}", *count, message));
    }

    fn get_count(&self) -> usize {
        *self.message_count.borrow()
    }

    fn print_history(&self) {
        println!("--- Log History ---");
        // We use .borrow() for read-only access.
        for entry in self.history.borrow().iter() {
            println!("{}", entry);
        }
        println!("-------------------");
    }
}

fn main() {
    // Wrap the MessageLogger in Rc to allow shared ownership.
    // Normally, data inside an Rc is immutable.
    let shared_logger: Rc<MessageLogger> = Rc::new(MessageLogger::new());

    // Create multiple references to the same logger instance.
    // These are just pointers to the same data on the heap.
    let logger_clone1 = Rc::clone(&shared_logger);
    let logger_clone2 = Rc::clone(&shared_logger);

    // Log messages from different references.
    // Even though logger_clone1 is an immutable variable holding an immutable reference,
    // we can still "change" the logger because of RefCell.
    shared_logger.log("Main logger event.");
    logger_clone1.log("Event from clone 1.");
    logger_clone2.log("Event from clone 2.");

    // Check the final state from any of the references.
    println!("\nTotal messages logged: {}", shared_logger.get_count());
    shared_logger.print_history();
}