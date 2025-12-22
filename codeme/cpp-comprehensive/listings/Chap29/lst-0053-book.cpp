
// https://godbolt.org/z/7E1E6aezE
#include <future>
#include <thread>
#include <iostream>
#include <exception>
int ack(int m, int n); // Ackermann function
void longComputation(std::promise<int> intPromise) {
  try {
    int result = ack(3,12);
    intPromise.set_value(result);                       // report result
  } catch (std::exception &e) {
    intPromise.set_exception(make_exception_ptr(e));    // report exception
  } catch ( ... ) {
    intPromise.set_exception(std::current_exception()); // Exc. w/o name
  }
}
int main () {
  std::promise<int> intPromise;                         // Create promise
  std::future<int> intFuture = intPromise.get_future(); // Request future
  std::jthread th{ longComputation,                     // start
             std::move(intPromise) };                   // Pass promise
  th.detach();                                          // let it run
  // might throw an exception:
  int result = intFuture.get();                         // Request result
  std::cout << result << std::endl;
}