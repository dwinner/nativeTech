//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9xrP8Ehvd 
#include <algorithm>
#include <vector>
#include <iostream>
void sort_uniq(std::vector<int> &data) {
    std::ranges::sort(data);                        // sorting
    auto to_delete = std::ranges::unique(data);     // move to the back
    data.erase(to_delete.begin(), to_delete.end()); // actually delete
}
int main() {
    std::vector ns{1,5,2,3,9,2,2,2,2,1,5,2,2,3,1,1,2,2,1};
    sort_uniq(ns);
    std::ranges::for_each(ns, [](auto x) {
        std::cout << x << ' '; });
    std::cout << '\n'; // Output: 1 2 3 5 9
}