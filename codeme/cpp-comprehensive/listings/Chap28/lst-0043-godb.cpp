//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xMz4s8K8K
#include <chrono>
#include <iostream>
using namespace std::chrono; using std::cout;
using seconds32 = duration<int32_t>;                // other representation
using ten_day = duration<int,std::ratio<86400*10>>; // other time unit
using fseconds = duration<double>;                  // Floating point representation
int main() {
  seconds32 s{5};
  cout << milliseconds(s).count() << "ms\n";
  ten_day z{1};
  hours h = z;                  // Conversion free
  cout << "1 ten_day has "<<h.count()<<" hours\n";                // …240…
  fseconds fs{23.75};
  cout << fs.count() << "s\n"; // Floating point output
  auto printDur = [](fseconds f) { cout << f.count() << "s\n"; }; // Function
  printDur(45ms + 63us);       // Conversion to fseconds
  // Output: 0.045063s
}