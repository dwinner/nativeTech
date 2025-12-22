
// https://godbolt.org/z/qzWchWoeK 
#include <vector>
#include <set>
#include <iostream> // cout
#include <concepts> // integral
#include <iterator> // output_iterator, input_iterator
using namespace std;
using vector_t = vector<unsigned long long>;   // your own type alias
int main() {
  vector_t huge{ 12ULL, 10000000000ULL, 9ULL, 0ULL,  };
  unsigned_integral auto sz = huge.size();
  unsigned_integral auto uiuiui = huge[1];
  signed_integral auto meh = huge[1];          // (ERR) concept not fulfilled
  input_or_output_iterator auto itx = huge.begin(); // concept without parameter
  for(output_iterator<unsigned long long> auto it=huge.begin();
      it!=huge.end(); ++it)
    *it *= 2; // double
  /* sort using set */
  set sorted(huge.begin(), huge.end());        // set<vector_t::value_type>
  for(const unsigned_integral auto& val : sorted)
    cout << val << " ";
  cout << "\n";
}