//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9s94nbrx3 
#include <future>
#include <thread>
#include <iostream>
int ack(int m, int n); // Ackermann function
int main () {
    std::packaged_task<int(void)> task1 {  // Signature of the remaining function
    []{ return ack(3,11);}  };             // prepare ack(3,11)
  auto f1 = task1.get_future();            // Communication channel
  std::jthread th1 { move(task1) };        // in new thread
  std::cout << "  ack(3,11):" << f1.get()  // Retrieve result
      << '\n';                             // Output: ack(3,11):16381
}