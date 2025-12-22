
// https://godbolt.org/z/a8cGxr9s9 
#include <unordered_set> // unordered_multiset
#include <iostream>
#include <string>
using std::unordered_multiset; using std::cout; using std::string;
int main() {
    const string in = "She sells sea shells by the seashore.";
    unordered_multiset<int> cs(in.begin(), in.end()); // string as container
    cout << cs.count( 's' ) << " times s\n"; // Output: 7 times s
}