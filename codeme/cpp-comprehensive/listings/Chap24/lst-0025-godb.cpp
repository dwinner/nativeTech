//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cE6G1aErs 
auto count(vector<int> arg) { return arg.size(); }
// …
vector input{1,2,3};           // vector<int>
vector outputA(input);         // copy
vector outputB = input;        // also copy, no assignment
cout << count(input) << '\n';  // call by copy-by-value