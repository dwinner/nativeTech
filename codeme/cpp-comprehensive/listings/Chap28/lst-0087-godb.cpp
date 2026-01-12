//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/GMjcjrvaT
#include <functional>
#include <iostream>
struct Numbers {
    int theNumber() {
        return 42;
    }
    int more(int n) {
        return n + data;
    }
    int data = 7;
};
int main() {
    auto func = std::mem_fn(&Numbers::theNumber);
    auto func2 = std::mem_fn(&Numbers::more);
    auto access = std::mem_fn(&Numbers::data);
    Numbers numbers;
    std::cout << func(numbers) << '\n';        // Output: 42
    std::cout << func2(numbers, 66) << '\n';   // Output: 73
    std::cout << access(numbers) << '\n';      // Output: 7
}