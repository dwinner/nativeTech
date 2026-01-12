//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ro6fdsvT9 
#include <vector>
#include <iostream> // cout
using std::vector; using std::cout;
int main() {
    vector data{ 12, 100, -1, 0,  };    // vector<int>
    for(auto& val : data)
        val *= 2;                       // double it
    for(const auto val : data)
        cout << val << " ";
    cout << "\n";
}