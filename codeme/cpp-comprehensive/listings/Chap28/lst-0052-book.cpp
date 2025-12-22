
// https://godbolt.org/z/ezn568nof
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
  auto spring = local_days{31d/March/2024} + 2h + 1min;
  try {
      auto zt = zoned_time{"Europe/Berlin", spring};
  } catch (const nonexistent_local_time& e) {
      std::cout << e.what() << '\n'; // Exception thrown: does not exist
  }
  auto autumn = local_days{27d/October/2024} + 2h + 1min;
  try {
      auto zt = zoned_time{"Europe/Berlin", autumn};
  } catch (const ambiguous_local_time& e) {
     std::cout << e.what() << '\n'; // Exception thrown: already exists
  }
  std::cout << zoned_time{"Europe/Berlin", autumn, choose::earliest} << '\n';
  // Output: 2024-10-27 02:01:00 CEST
  std::cout << zoned_time{"Europe/Berlin", autumn, choose::latest} << '\n';
  // Output: 2024-10-27 02:01:00 CET
}