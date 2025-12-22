
// https://godbolt.org/z/bq1hn5rjo 
#include <future>
#include <thread>
#include <iostream>
int ack(int m, int n); // Ackermann function
int main () {
  std::packaged_task<int(int,int)> task2 { &ack }; // different signature
  auto f2 = task2.get_future();
  std::jthread th2 { move(task2), 3, 12 };         // Parameters here
  std::cout << "  ack(3,12):" << f2.get() << '\n'; // Output: ack(3,12):32765
  th2.join();
}