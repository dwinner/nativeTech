//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/d3ExcWP4a
#include <deque>
#include <iostream>
#include <iterator>  // ostream_iterator
#include <algorithm> // copy
using namespace std;
ostream& operator<<=(ostream& os, int val) { return os<<val<<'\n'; }
int main() {
    deque d{ 4, 6 };            // Create deque<int> with elements
    // Insertion
    d.push_front(3);            // at the front: efficient
    d.insert(d.begin()+2, 5);   // in the middle: slow
    d.push_back(7);             // at the end: efficient
    // Access
    cout <<= d.front();         // from the front: efficient
    cout <<= d[3];              // in the middle: efficient
    cout <<= d.back();          // from the end: efficient
    // Size
    cout <<= d.size();          // read size
    d.resize(4);                // resize (or extend)
    // Iterators
    copy(d.begin(), d.end(), ostream_iterator<int>(cout, " "));
    cout << '\n';               // Output: 3 4 5 6
    // Remove
    d.pop_front();              // at the front: efficient
    d.erase(d.begin()+1);       // in the middle: slow
    d.pop_back();               // at the end: efficient
    d.clear();                  // clear
}