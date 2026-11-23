// 10_rc_refcell_combination.rs

use std::rc::Rc;
use std::cell::RefCell;

// ==========================================
// PART 1: The Observer Pattern (Complex Use)
// ==========================================

// A simple trait for observers
trait Observer {
    fn update(&self, new_value: i32); 
}

// The subject that observers will watch.
// We use RefCell for fields so we can mutate them even with immutable references (&self).
struct Subject {
    value: RefCell<i32>, 
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    // Note: We are returning a plain Subject here for simplicity, 
    // relying on the internal RefCells for mutability.
    fn new(initial_value: i32) -> Self { 
        Subject {
            value: RefCell::new(initial_value),
            observers: RefCell::new(Vec::new()),
        }
    }

    // Takes &self (immutable), but mutates the observers list via RefCell
    fn attach(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    // Takes &self (immutable), but mutates value via RefCell
    fn set_value(&self, new_value: i32) {
        *self.value.borrow_mut() = new_value;
        println!("Subject: Value changed to {}", new_value);

        // Notify all observers
        // We borrow() the list immutably to iterate
        for observer in self.observers.borrow().iter() {
            observer.update(new_value);
        }
    }
}

// A concrete observer
struct ValueMonitor {
    id: String,
    last_seen_value: RefCell<Option<i32>>, // Internal mutable state
}

impl ValueMonitor {
    fn new(id: &str) -> Rc<Self> {
        Rc::new(ValueMonitor {
            id: id.to_string(),
            last_seen_value: RefCell::new(None),
        })
    }
}

impl Observer for ValueMonitor {
    fn update(&self, new_value: i32) {
        // We can mutate our internal state despite `&self` thanks to RefCell
        *self.last_seen_value.borrow_mut() = Some(new_value);
        println!("Observer '{}': Notified of new value -> {}", self.id, new_value);
    }
}

fn main() {
    println!("--- Example 1: Observer Pattern with RefCell ---");

    let subject = Subject::new(0);
    
    // Create observers wrapped in Rc so they can be owned by main AND the subject
    let monitor1 = ValueMonitor::new("MonitorA");
    let monitor2 = ValueMonitor::new("MonitorB");

    // Attach observers (cast to the Trait Object)
    subject.attach(Rc::clone(&monitor1) as Rc<dyn Observer>);
    subject.attach(Rc::clone(&monitor2) as Rc<dyn Observer>);

    // Change value. This triggers the notifications.
    subject.set_value(42);


    println!("\n--- Example 2: Simple Shared Mutable List ---");
    // This demonstrates the specific logic found at the end of your snippet.
    // T is the data we want to share and mutate (Vec<String>).
    // Multiple owners get Rc<RefCell<T>>.
    
    let shared_list: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    // Create multiple owners
    let list_owner1 = Rc::clone(&shared_list);
    let list_owner2 = Rc::clone(&shared_list);

    // Owner 1 adds to the list
    list_owner1.borrow_mut().push("Hello from owner 1".to_string());
    println!("Owner 1 added. List: {:?}", list_owner1.borrow());

    // Owner 2 adds to the SAME list
    list_owner2.borrow_mut().push("Owner 2 was here".to_string());
    println!("Owner 2 added. List: {:?}", list_owner2.borrow());

    // The original reference also sees the changes
    println!("Main sees list: {:?}", shared_list.borrow());
}