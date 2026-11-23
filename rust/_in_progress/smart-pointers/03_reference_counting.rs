// 03_reference_counting.rs
use std::rc::Rc;

#[derive(Debug)]
struct SharedConfig {
    version: String,
    api_key: String,
}

struct ServiceA {
    id: u32,
    // We use Rc here because ServiceA needs to share ownership of the config.
    // It doesn't own it exclusively, so Box<T> wouldn't work (Box implies unique ownership).
    config: Rc<SharedConfig>,
}

struct ServiceB {
    name: String,
    config: Rc<SharedConfig>,
}

fn main() {
    // Create the shared configuration data wrapped in an Rc.
    // This puts the data on the heap and initializes the reference count to 1.
    let shared_config = Rc::new(SharedConfig {
        version: "v1.2.3".to_string(),
        api_key: "ABC123XYZ789".to_string(),
    });

    println!("Initial strong count: {}", Rc::strong_count(&shared_config));

    // Create clones of the Rc to share ownership.
    // Rc::clone(&shared_config) is preferred over shared_config.clone().
    // It makes it visually obvious that we are only incrementing the counter,
    // not performing a deep copy of the data.
    let service_a = ServiceA {
        id: 101,
        config: Rc::clone(&shared_config),
    };
    println!("After ServiceA created: strong count = {}", Rc::strong_count(&shared_config));

    let service_b = ServiceB {
        name: "LoggerService".to_string(),
        config: Rc::clone(&shared_config),
    };
    println!("After ServiceB created: strong count = {}", Rc::strong_count(&shared_config));

    println!("---");
    // We can access the data inside via auto-dereferencing
    println!("Service A accesses config version: {}", service_a.config.version);
    println!("Service B accesses API key: {}", service_b.config.api_key);
    println!("---");

    // Explicitly drop service_a to see the reference count decrease.
    drop(service_a);
    println!("After ServiceA is dropped: strong count = {}", Rc::strong_count(&shared_config)); // Output: 2

    // Explicitly drop service_b.
    drop(service_b);
    println!("After ServiceB is dropped: strong count = {}", Rc::strong_count(&shared_config)); // Output: 1

    // The final Rc (`shared_config`) will be dropped at the end of main's scope.
    // At that point, the count will become 0, and the SharedConfig data will be deallocated.
}