
// https://godbolt.org/z/EnjPr5r76
#include <atomic>
class SpinlockMutex {
  std::atomic_flag flag_;
public:
  SpinlockMutex()
  : flag_{ATOMIC_FLAG_INIT}
  {}
  void lock() {                               // e.g., called by lock_guard
    while(flag_.test_and_set(std::memory_order_acquire)) // mostly read
      { /* nothing */ }
  }
  void unlock() {
    flag_.clear(std::memory_order_release);              // write operation
  }
};