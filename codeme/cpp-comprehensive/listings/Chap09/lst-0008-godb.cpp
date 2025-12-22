//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EY5K6Wsr3 
std::vector<int> data(10);     // 10 times 0 in a vector
data.front() = 666;            // writes 666 to the front position