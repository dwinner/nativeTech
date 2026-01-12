
// https://godbolt.org/z/Wh71cf7j1 
#include <vector>
#include <set>
#include <iostream> // cout
using std::vector; using std::set; using std::cout;
using vector_t = vector<unsigned long long>; // Your own type alias
int main() {
    vector_t huge{ 12ULL, 10000000000ULL, 9ULL, 0ULL,  };
    auto sz = huge.size();
    auto uiuiui = huge[1];
    for(auto it = huge.begin(); it != huge.end(); ++it)
        *it *= 2; // double
    /* sort via set */
    set sorted(huge.begin(), huge.end());  // set<vector_t::value_type>
    for(auto val : sorted)
        cout << val << " ";
    cout << "\n";
}