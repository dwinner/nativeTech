// 15_test_doubles_and_stubs.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file:
// 1. Compile: rustc --test 15_test_doubles_and_stubs.rs -o test_runner
// 2. Run:     ./test_runner
// ---------------------------------------------------------

// --- The Interface (Trait) ---
// This defines the behavior we depend on.
pub trait Notifier {
    fn send_alert(&self, user_id: &str, message: &str) -> Result<(), String>;
}

// --- The Component Under Test ---
// We use Generics <N: Notifier> to allow injecting ANY notifier.
pub struct EventProcessor<N: Notifier> {
    // We make this pub so our tests can inspect the stub state later.
    pub notifier_service: N, 
    admin_user_id: String,
}

impl<N: Notifier> EventProcessor<N> {
    pub fn new(notifier_service: N, admin_user_id: String) -> Self {
        EventProcessor { notifier_service, admin_user_id }
    }

    pub fn process_critical_event(&self, event_details: &str) {
        println!("Processing critical event: {}", event_details);
        
        // Attempt to notify the admin
        let alert_message = format!("CRITICAL: {}", event_details);
        
        match self.notifier_service.send_alert(&self.admin_user_id, &alert_message) {
            Ok(_) => println!("Admin notified successfully."),
            Err(e) => println!("Failed to notify admin: {}", e),
        }
    }
}

// --- The Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    // Our Stub implementation for the Notifier trait.
    // This acts as a "Test Double".
    struct StubEmailNotifier {
        should_succeed: bool,
        expected_user_id: String,
        expected_message_contains: String,
        // We use Cell because 'send_alert' takes &self (immutable),
        // but we want to modify call_count to track usage.
        call_count: Cell<usize>, 
    }

    impl Notifier for StubEmailNotifier {
        fn send_alert(&self, user_id: &str, message: &str) -> Result<(), String> {
            // Increment call count using Interior Mutability
            self.call_count.set(self.call_count.get() + 1); 
            
            println!("STUB: Attempting to send alert to '{}'", user_id);
            
            // Assertions inside the stub ensure the component called us with correct arguments
            assert_eq!(user_id, self.expected_user_id, "Stub called with wrong user_id");
            assert!(message.contains(&self.expected_message_contains), "Stub message content mismatch");

            if self.should_succeed {
                Ok(())
            } else {
                Err("StubNotifier: Simulated failure.".to_string())
            }
        }
    }

    #[test]
    fn critical_event_notifies_admin_successfully() {
        // 1. Setup the Stub
        let stub_notifier = StubEmailNotifier {
            should_succeed: true,
            expected_user_id: "admin_001".to_string(),
            expected_message_contains: "CRITICAL: System Overload".to_string(),
            call_count: Cell::new(0),
        };

        // 2. Inject the Stub into the Processor
        let event_processor = EventProcessor::new(stub_notifier, "admin_001".to_string());

        // 3. Trigger the logic
        event_processor.process_critical_event("System Overload");

        // 4. Verify the interaction
        // Because 'notifier_service' is stored in the struct, we can check if it was called.
        assert_eq!(
            event_processor.notifier_service.call_count.get(), 
            1, 
            "Notifier should have been called exactly once"
        );
    }

    #[test]
    fn critical_event_handles_notification_failure() {
        let stub_notifier = StubEmailNotifier {
            should_succeed: false, // Simulate failure
            expected_user_id: "sys_alert_user".to_string(),
            expected_message_contains: "CRITICAL: Disk Full".to_string(),
            call_count: Cell::new(0),
        };

        let event_processor = EventProcessor::new(stub_notifier, "sys_alert_user".to_string());

        // This should print a failure message to stdout, but should NOT panic.
        event_processor.process_critical_event("Disk Full");

        assert_eq!(
            event_processor.notifier_service.call_count.get(), 
            1, 
            "Notifier should have been called once, even on failure path"
        );
    }
}

fn main() {
    println!("This file demonstrates Test Doubles (Stubs/Mocks).");
    println!("Run with `rustc --test 15_test_doubles_and_stubs.rs`");
}