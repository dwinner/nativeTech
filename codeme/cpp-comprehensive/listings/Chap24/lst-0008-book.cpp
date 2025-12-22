
// https://godbolt.org/z/M1d9jEP74 
#include <vector>
#include <iostream>
using namespace std;
int main() {
    vector numbers{ 1,2,3,4,5 };
    numbers.erase(numbers.begin(), numbers.end());
    cout << numbers.size() << "\n"; // Output: 0
}