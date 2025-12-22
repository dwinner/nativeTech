//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/aqGTKdbMP
#include <vector>
// …
multiset msinit{1,2,2,3,1};                 // sorted at initialization
cout <<= msinit;        // Output: 1 1 2 2 3
std::vector in{ 7,7,7,7,7,7,7 };
std::set srange( in.begin(), in.end() );    // set removes duplicates
cout << srange.size() << ": " << *srange.begin() << '\n'; // Output: 1: 7
multiset msrange( in.begin(), in.end() );   // multiset retains duplicates
cout <<= msrange;       // Output: 7 7 7 7 7 7 7