
// https://godbolt.org/z/nj14e6Wq4 
#include <set>
#include <iostream>
int main() {
    std::set<int> numbers{ 10, 20, 90 };
    auto no = numbers.find(30);
    if(no == numbers.end()) { std::cout << "not there.\n"; }
    auto yes = numbers.find(20);
    if(yes != numbers.end()) { std::cout << *yes << '\n'; }
}