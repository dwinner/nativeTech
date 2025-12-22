//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1dTPEd3nP 
set source{1,2,3,4,5};
cout <<= source;         // Output: 1 2 3 4 5
set target( std::move(source) ); // move instead of copy
cout <<= source;         // Output:
cout <<= target;         // Output: 1 2 3 4 5