//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ozMY86ns8
#include <vector>
// …
set data{1,2,3,4,5};
std::vector source{10, 20, 30, 40, 50};

// There is no set::assign:
data.assign(source.begin(), source.end());   // (ERR) no set::assign
// So simulate it using a temporary set:
set temp(source.begin(), source.end());      // copy from source …
data.swap(temp);                             // … efficiently swap contents
cout <<= data;                  // Output: 10 20 30 40 50
// … or by clearing first and then inserting:
data.clear();                                // clear …
data.insert(source.begin(), source.end());   // … and insert
cout <<= data;                  // Output: 10 20 30 40 50