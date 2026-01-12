//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qGrYPrqYn 
#include <vector>
#include <iostream>
using namespace std;
int main() {
    vector nums{ 1,2,3 };
    vector four{ 4,5,6 };
    vector seven{ 7,8,9 } ;
    nums.insert(nums.begin(), four.begin(), four.end()); // pair of iterators
    cout << nums.size() << "\n";                         // Output: 9
    nums.insert_range(nums.begin(), seven);              // C++23: range
}