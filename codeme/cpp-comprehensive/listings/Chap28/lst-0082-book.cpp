
// https://godbolt.org/z/jxvhded75
#include <string>
#include <vector>
#include <iostream>
#include <map>
#include <functional>
std::map<char,std::function<int(int,int)>> binOps { // binary operators
    {'+', std::plus<int>{} },
    {'-', std::minus<int>{} },
    {'*', std::multiplies<int>{} },
    {'/', std::divides<int>{} },
    {'%', std::modulus<int>{} },
    };
std::map<char,std::function<int(int)>> unOps { };    // unary operators
auto val = [](auto n) { return [n](){ return n; };}; // returns a lambda
std::map<char,std::function<int()>> zeroOps {        // nullary operators
  {'0', val(0)}, {'1', val(1)}, {'2', val(2)}, {'3', val(3)}, {'4', val(4)},
  {'5', val(5)}, {'6', val(6)}, {'7', val(7)}, {'8', val(8)}, {'9', val(9)},
  };
std::map<char,std::function<void(std::vector<int>&)>> stackOps { 
  { ' ', [](auto &stack) { } },           // no operation
  { 'c', [](auto &stack) { stack.clear(); } }, // Clear the entire stack
  { ':', [](auto &stack) {                // swap the top two elements
            auto top = stack.back(); stack.pop_back();
            auto second = stack.back(); stack.pop_back();
            stack.push_back(top);
            stack.push_back(second);
  } },
  { '=', [](auto &stack) {                // print the entire stack
            for(int elem : stack) { std::cout << elem; }
            std::cout << "\n";
  } },
};
void calculator(std::string input) {
  std::vector<int> stack {};
  for(char c : input) {
    int top, second;
    if(auto it = unOps.find(c); it != unOps.end()) {
      // if unary operator …
      auto func = it->second;
      top = stack.back(); stack.pop_back(); // … get top element
      stack.push_back(func(top));           // … apply func, push result onto stack
    } else if(auto it = binOps.find(c); it != binOps.end()) {
      // if binary operator …
      auto func = it->second;
      top = stack.back(); stack.pop_back(); // … get the top 2 elements
      second = stack.back(); stack.pop_back();
      stack.push_back(func(second, top));// … apply func, push result onto stack
    } else if(auto it = zeroOps.find(c); it !=zeroOps.end()) {
      // if nullary operator …
      auto func = it->second;
      stack.push_back(func());               // ... result of func on stack
    } else if(auto it = stackOps.find(c); it != stackOps.end()) {
      // if stack operator
      auto func = it->second;
      func(stack);                           // ... apply func to stack
    } else {
      std::cout << "\n'" << c << "' I don't understand.\n";
    }
  } /* for c */
}
int main(int argc, const char* argv[]) {
    if(argc > 1) {
        calculator(argv[1]);
    } else {
        // 3+4*5+6 with multiplication before addition results in 29
        calculator("345*+6+=");
    }
    calculator("93-=");                      // 9 - 3 = Output: 6
    calculator("82/=");                      // 8 / 2 = Output: 4
    calculator("92%=");                      // 9 % 2 = Output: 1
}