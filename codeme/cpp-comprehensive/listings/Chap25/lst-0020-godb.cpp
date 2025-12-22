//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Y7GPcqWM8
#include <numeric>    // accumulate
#include <functional> // multiplies
#include <algorithm>  // transform, fold_left
#include <iostream>
#include <vector>
using std::accumulate; using std::cout; using std::vector; using std::multiplies;
namespace rs = std::ranges;
int main() {
  vector data{ 2, 3, 5, 10, 20 };
  cout << accumulate(data.begin(),data.end(),0)<<'\n';     // +, Output: 40
  cout << rs::fold_left(data, 1, multiplies<int>{})<<'\n'; // *, 6000
  vector<bool> even( data.size() );
  std::transform( data.begin(), data.end(), even.begin(),
      [](auto n) { return n%2==0; });
  for(auto b : even) {
      cout << ( b ? "even " : "odd ");
  }
  cout << "\n";        // Output: even odd odd even even
  auto areAllEven = accumulate(even.begin(), even.end(), true,
          [](auto b, auto c) { return b&&c; });
  if(areAllEven) {
      cout << "all even numbers\n";
  } else {
      cout << "odd numbers included\n"; // this is the output
  }
}