//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/andfoh18z
#include <forward_list>
#include <iostream>
#include <iterator> // next
using std::cout; using std::forward_list; using std::ostream;
ostream& operator<<=(ostream&os, const forward_list<int> &data)
    { for(auto &e:data) os<<e<<' '; return os<<'\n'; }

int main()     {
    forward_list nums {40, 50, 60, 70};
    cout <<= nums;                         // Output: 40 50 60 70
    nums.insert_after(nums.before_begin(), {10, 20, 30});
    cout <<= nums;                         // Output: 10 20 30 40 50 60 70
    auto wh = std::next(nums.begin(), 2);  // two elements forward
    auto to = std::next(wh, 3);            // three elements after where
    nums.erase_after(wh, to);
    cout <<= nums;                         // Output: 10 20 30 60 70
}