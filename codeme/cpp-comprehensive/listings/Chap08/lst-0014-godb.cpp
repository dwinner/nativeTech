//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xWbqeeq3n
#include <iostream>
#include <tuple>   // make_tuple
auto mkTpl() {
    return std::make_tuple(2, 'b', 3.14);  // tuple<int,char,double>
}
struct Point {
    int x, y;
};
int main() {
    // Structured binding of a C-array
    int odd[5] = { 1,3,7,9,11 };
    auto [ one, two, three, four, five ]  = odd;
    // Structured binding of a tuple
    auto [ zwei, be, pi ]  = mkTpl();
    // Structured binding of a struct
    Point p0{  10, 15 };
    auto [ the_x, the_y ]  = p0;
}