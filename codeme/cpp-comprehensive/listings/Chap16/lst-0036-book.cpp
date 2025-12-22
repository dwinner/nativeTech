
// https://godbolt.org/z/KxWb4q8q9 
#include <compare>  // partial_ordering etc
#include <set>
#include <iostream>
using namespace std;
struct Fraction {
  int z, n;  // numerator / denominator
  partial_ordering operator<=>(const Fraction& rhs) const {
    if(n==0 || rhs.n==0) return partial_ordering::unordered;
    return (double)z / n <=> (double)rhs.z / rhs.n;
  }
};
int main() {
  set<Fraction> fractions{ {1,2}, {2,4}, {1,3}, {2,3}, {1,4}, {2,5}, {3,8}, {99,0} };
  for(auto f : fractions)
    cout << f.z << "/" << f.n << " ";
  cout << "\n”; // Output: 1/4 1/3 3/8 2/5 1/2 2/3
}