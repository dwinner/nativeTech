
// https://godbolt.org/z/Ka47W5Wjh 
#include <forward_list>
#include <iostream>
using std::cout; using std::forward_list; using std::ostream;

ostream& operator<<=(ostream&os, const forward_list<int> &data)
  { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main() {
  {
    forward_list fw1 {10, 20, 30, 40};
    forward_list fw2 {5, 6, 7, 8};
    fw1.splice_after(fw1.begin(), fw2);  // transfers everything
    cout <<= fw1;                        // Output: 10 5 6 7 8 20 30 40
    cout <<= fw2;                        // Output:
  }
  {
    forward_list fw1 {10, 20, 30, 40};
    forward_list fw2 {5, 6, 7, 8};
    // one element left:
    fw1.splice_after(fw1.begin(), fw2,fw2.begin(),fw2.end());
    cout <<= fw1;                        // Output: 10 6 7 8 20 30 40
    cout <<= fw2;                        // Output: 5
  }
}