//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/PeG4K7Edc
valarray v { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 };
valarray<int> r1(v[slice(0, 4, 3)]); // Start at 0, 4 elements, step size 3
cout <<= r1;                         // Output: 1 4 7 10
valarray<int> r2(v[v > 6]);          // addressed by valarray<bool>
cout <<= r2;                         // Output: 7 8 9 10 11 12
const valarray<size_t> indirect{ 2, 2, 3, 6 };  // duplicates allowed
valarray<int> r5(v[indirect]);       // addressed by valarray<size_t>
cout <<= r5;                         // Output: 3 3 4 7