
// https://godbolt.org/z/4YM9d99xG 
#include <iostream>
#include <string>
#include <vector>
#include <utility> // pair
using std::pair; using std::cout; using std::cin; using std::string;
std::vector<string> months { "Jan", "Feb", "Mar" };
std::vector temps { 8, 12, 11 };
std::pair<string, int> monthWithTemp(size_t m) {
    auto month = months.at(m);
    auto temperature = temps.at(m);
    return std::make_pair(month, temperature);
}
int main() {
    std::pair data = monthWithTemp(1);
    cout << "Month : " << data.first << std::endl; // Output: Month : Feb
    cout << "Temperature : " << data.second << std::endl; 
    // Output: Temperature : 12
}