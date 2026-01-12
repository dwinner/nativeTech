
// https://godbolt.org/z/1j6PMojWr
#include <set>
template<typename Key>
std::set<Key> sorted(const unordered_set<Key> &data)
  { return std::set<Key>(data.begin(), data.end()); }
// …
// without arguments
unordered_set<int> empty{};
cout <<= empty;    // Output:
// initializer list
unordered_set data{1,1,2,2,3,3,4,4,5,5}; // duplicates are not included
cout <<= data;     // Output approximately: 5 4 3 2 1
// copy
unordered_set copy(data);
cout <<= copy;     // Output approximately: 5 4 3 2 1
// Range
auto so1 = sorted(data);
unordered_set range(std::next(so1.begin()), std::prev(so1.end()));
cout <<= range;    // Output approximately: 2 3 4