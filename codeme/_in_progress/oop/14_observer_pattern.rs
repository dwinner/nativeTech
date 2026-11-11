// 14_observer_pattern.rs

use std::rc::Rc;
use std::cell::RefCell;

// The trait that all observers must implement.
trait Observer {
    // The subject calls this method to notify the observer of a change.
    fn update(&self, new_state: &str);
}

// The subject holds the state and a list of observers.
struct Subject {
    state: String,
    // We use Rc<RefCell<...>> to allow shared ownership and interior mutability of the observers list.
    // Rc: Allows the Observer to be owned by main AND held by the list.
    // RefCell: Allows us to mutate the Vec (add observers) even if we only have an immutable reference to Subject.
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    fn new(initial_state: &str) -> Self {
        Subject {
            state: initial_state.to_string(),
            observers: RefCell::new(Vec::new()),
        }
    }

    // Add a new observer to the list.
    // Note: We take &self (immutable), but modify observers using borrow_mut() (Interior Mutability).
    fn attach(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    // Change the state and notify all observers.
    fn set_state(&mut self, new_state: &str) {
        self.state = new_state.to_string();
        println!("\nSubject: State changed to '{}'. Notifying observers...", self.state);
        
        // We borrow the observers list immutably to iterate and notify.
        for observer in self.observers.borrow().iter() {
            observer.update(&self.state);
        }
    }
}

// A concrete observer that logs updates.
struct Logger {
    name: String,
}

impl Observer for Logger {
    fn update(&self, new_state: &str) {
        println!("[Logger {}]: Received update! New state is: '{}'", self.name, new_state);
    }
}

// Another concrete observer that might perform a different action.
struct Notifier {
    email: String,
}

impl Observer for Notifier {
    fn update(&self, new_state: &str) {
        println!("[Notifier]: Sending email to {}. Subject: State changed to '{}'", self.email, new_state);
    }
}

fn main() {
    let mut subject = Subject::new("Initial State");

    // Create observers. We wrap them in Rc to manage shared ownership.
    let logger = Rc::new(Logger { name: "FileLogger".to_string() });
    let notifier = Rc::new(Notifier { email: "admin@example.com".to_string() });

    // Attach the observers to the subject.
    // We clone the Rc (incrementing the reference count), not the object itself.
    // We cast to Rc<dyn Observer> to store them in the heterogeneous list.
    subject.attach(Rc::clone(&logger) as Rc<dyn Observer>);
    subject.attach(Rc::clone(&notifier) as Rc<dyn Observer>);

    // Change the subject's state. This should trigger notifications.
    subject.set_state("State A");
    subject.set_state("State B");
}