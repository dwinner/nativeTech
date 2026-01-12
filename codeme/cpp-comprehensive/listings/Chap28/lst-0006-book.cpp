
// https://godbolt.org/z/9qhMo6zbb 
#include <iostream>
#include <string>
#include <tuple>
#include <map>
#include <unordered_set>

using std::tuple; using std::get; using std::cout; using std::string;

int main() {
    std::map<tuple<int,string>,double> m { {{12,"x"},3.14} };
    cout << m[{12,"x"}] << "\n"; // Output: 3.14
    std::unordered_set<tuple<int,string>> s { {12,"x"} }; // (ERR) no std::hash
}