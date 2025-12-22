//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4nqz4ed4W 
#include <vector>
#include <iostream> // cout

using std::vector;
int main() {
    vector data{ 5,4,3,2,1 };
    vector<int>::const_iterator end = data.end(); // or end(data)
    for(vector<int>::const_iterator it = data.begin(); it!=end; ++it) {
        std::cout << *it << " ";
    }
    std::cout << "\n";
}