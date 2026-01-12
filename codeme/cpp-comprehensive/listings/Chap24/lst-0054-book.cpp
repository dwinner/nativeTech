
// https://godbolt.org/z/YzGK1WazE
#include <vector>
#include <iostream>
#include <format> // C++20
using namespace std;
ostream& operator<<(ostream&os, const vector<int> &vec) {
  for(auto &elem : vec) { os << elem << ' '; } return os;
}
int main() {
  vector<int> data {};      // creates an empty vector
  data.reserve(3);          // Space for 3 elements
  cout << format("{}/{}\n", data.size(), data.capacity()); // 0/3
  data.push_back(111);      // add an element
  data.resize(3);           // Resize; here it appends new elements
  data.push_back(999);      // add another element
  cout << format("{}/{}\n", data.size(), data.capacity()); // 4/6
  cout << data << '\n';     // 111, 0, 0, 999
  data.push_back(333);      // add another element
  cout << data << '\n';     // 111, 0, 0, 999, 333
  data.reserve(1);          // nothing happens, because capacity() > 1
  data.resize(3);           // shrink; elements are removed
  cout << data << '\n';     // 111, 0, 0
  cout << format("{}/{}\n", data.size(), data.capacity()); // 3/6
  data.resize(6, 44);       // resize again, fill with 44s
  cout << data << '\n';     // 111, 0, 0, 44, 44, 44
}