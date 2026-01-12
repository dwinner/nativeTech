//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8enx43Ksb
#include <coroutine>
#include <iostream>
struct Routine { // Coroutine return type
  struct promise_type;                     // defined further ahead
  Routine(auto h) : hdl_(h) {}             // Constructor
  ~Routine() { if(hdl_) hdl_.destroy(); }  // Destructor
  Routine(const Routine&) = delete;        // no copies
  Routine(Routine&&) = delete;             // no moves
  // Interface for the caller:
  bool more() {                            // continue coroutine
    if(!hdl_ || hdl_.done()) return false; // - nothing more to do
    hdl_.resume();                         // - blocking call
    return !hdl_.done();                   // - signals done or not
  }
private:
  using handle_type_ = std::coroutine_handle<promise_type>;
  handle_type_ hdl_;     // to control the coroutine
};
struct Routine::promise_type {             // necessary promise_type
  auto get_return_object() {               // initializes the handle
    return Routine{ handle_type_::from_promise(*this) };
  }
  auto initial_suspend() { return std::suspend_never{}; } // start immediately
  auto final_suspend() noexcept { return std::suspend_always{}; } // normal case
  void unhandled_exception() {}            // no exception handling
  void return_void() {}                    // Coroutine has no return
};
Routine counter() {  // Return type is our wrapper class with the promise_type
  for (unsigned i = 0;; ++i) {
    co_await std::suspend_always{};        // Pause coroutine
    std::cout << "counter: " << i << std::endl;
  }
}

int main() {
  Routine routine = counter();             // Create and start coroutine
  for (int i = 0; i < 3; ++i) {
      std::cout << "Hello main!\n";
      routine.more();                      // continue coroutine
  }
}