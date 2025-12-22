
// https://godbolt.org/z/1dTPEd3nP 
set source{1,2,3,4,5};
cout <<= source;         // Output: 1 2 3 4 5
set target( std::move(source) ); // move instead of copy
cout <<= source;         // Output:
cout <<= target;         // Output: 1 2 3 4 5