//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1WG5arvWT
#include <future> // async
#include <iostream>
using namespace std;
int calcHeight(int count, int maxCount, int scale) {
  if(maxCount == 0)
      throw logic_error("maxCount is 0");
  return (count * scale) / maxCount;
}

int main() {
  auto fut = async(launch::async, calcHeight,0,0,200); // (ERR) throws
  try {
    cout << fut.get() << '\n';              // triggers exception
  } catch(exception &ex) {
    cout << "Error: " << ex.what() << '\n'; // Output: Error: maxCount is 0
  }
}