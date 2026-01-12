//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/1esod6hrT
#include <algorithm>
#include <iostream>
#include <list>
#include <string>
#include <iterator> // ostream_iterator
#include <cctype>   // toupper
using std::toupper;
int main() {
    std::list a{ 1,2,4,4,4,7,7,9 };
    std::list b{ 2,2,3,4,4,8 };
    using Os = std::ostream_iterator<int>; // Type of output iterator
    Os os{std::cout, " "};                 // Stream output iterator for int
    auto run = [&a,&b,&os](auto algo) {    // use a, b, and os
        algo(a.begin(), a.end(), b.begin(), b.end(), os); // Call algorithm
        std::cout << '\n';
    };

    // Results of the algorithms
    using It = decltype(a.begin());        // Type of input iterators

    run(std::merge<It,It,Os>);             // Output: 1 2 2 2 3 4 4 4 4 4 7 7 8 9
    run(std::set_union<It,It,Os>);         // Output: 1 2 2 3 4 4 4 7 7 8 9
    run(std::set_intersection<It,It,Os>);  // Output: 2 4 4
    run(std::set_difference<It,It,Os>);    // Output: 1 4 7 7 9
    run(std::set_symmetric_difference<It,It,Os>); // Output: 1 2 3 4 7 7 8 9

    // With letters it becomes even clearer
    std::string x = "abdddggi";
    std::string y = "BBCDDH";
    using Us = std::ostream_iterator<char>;   // Type of output iterator
    Us us{std::cout, ""};                     // Stream output iterator for char
    auto compare = [](auto c, auto d) { return toupper(c) < toupper(d); };
    auto run2 = [&x,&y,&us,&compare](auto algo) {    // use x, y, and us
        algo(x.begin(), x.end(), y.begin(), y.end(), us, compare);
        std::cout << '\n';
    };
    // Results of the algorithms
    using Jt = decltype(x.begin());           // Type of input iterators
    using Cm = decltype(compare);             // Type of comparison function

    run2(std::merge<Jt,Jt,Us,Cm>);            // Output: abBBCdddDDggHi
    run2(std::set_union<Jt,Jt,Us,Cm>);        // Output: abBCdddggHi
    run2(std::set_intersection<Jt,Jt,Us,Cm>); // Output: bdd
    run2(std::set_difference<Jt,Jt,Us,Cm>);   // Output: adggi
    run2(std::set_symmetric_difference<Jt,Jt,Us,Cm>); // Output: aBCdggHi
}