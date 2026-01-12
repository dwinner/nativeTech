//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/fv5WEe97M
#include <list>
#include <string>
#include <iterator>                     // make_move_iterator
using std::make_move_iterator; using std::string;
vector<int> create() { return vector<int>{8, 9, 10}; }
size_t count(vector<int> d) { return d.size(); }
// …
vector input{1,2,3};
vector outputA(std::move(input));       // move
vector outputB = std::move(input);      // also move, not assignment
vector data = create();                 // Return-Value-Optimization
cout << count(input) << '\n';           // call by Copy-by-Value
// move elements from another container
std::list<string> source{ "a", "a", "a", "BB", "CC", "DD", "b", "b" };
auto from = source.begin();
std::advance(from, 3); // 3 forward, but slow in list
auto to = from;
std::advance(to, 3);   // another 3 forward
vector target(make_move_iterator(from), make_move_iterator(to));
// source is now {"a", "a", "a", "", "", "", "b", "b"}, target is now {"BB", "CC", "DD"}