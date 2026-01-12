//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/xE1TWx1ve
#include <map>
#include <memory> // unique_ptr
#include <string>
#include <iostream>
#include <chrono> // time measurement
using std::map; using std::cout; using std::endl; using namespace std::chrono;
struct Node {
  std::unique_ptr<int> d_;
  Node() : Node{0}  { }
  explicit Node(int d) : d_{ new int } { *d_ = d; }     // also some memory
  friend bool operator<(const Node& a, const Node& b) {return *a.d_<*b.d_;}
  friend bool operator==(const Node& a, const Node& b) {return *a.d_==*b.d_;}
};
long long millisSince(steady_clock::time_point start) { // Helper for time measurement
  return duration_cast<milliseconds>(steady_clock::now()-start).count();
}

int main() {
  std::unique_ptr<map<int,Node>> huge{ new map<int,Node>{} };
  cout << "Building..." << endl;
  steady_clock::time_point start = steady_clock::now();
  for(int idx=0; idx < 100*1000*1000; ++idx) {   // massive amount in the map
      (*huge)[idx] = Node{idx};
  }
  cout << "Done: " << millisSince(start) << " ms" << endl; // timing here
  start = steady_clock::now();
  huge.reset();                                            // cleanup here
  cout << "End: " << millisSince(start) << " ms" << endl;  // timing here
}