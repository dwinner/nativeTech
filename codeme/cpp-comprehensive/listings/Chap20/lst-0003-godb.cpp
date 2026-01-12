//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4536hxvEK 
int value = 42;
int& valueRef = value;  // Reference; no address operator & needed
valueRef = 18;          // no dereference operator * needed
cout << value << "\n";  // Output: 18