//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/ehP47vW9o 
valarray<int> v {
  1,  2,  3,
  4,  5,  6,
  7,  8,  9,
 10, 11, 12 };
v[slice(0, 4, 3)] *= valarray<int>(v[slice(0, 4, 3)]); // square the first column
cout <<= v;  // Output: 1 2 3 16 5 6 49 8 9 100 11 12
v[slice(0, 4, 3)] = valarray<int>{1, 4, 7, 10}; // restore
valarray<int> r3(v[gslice(0, {2, 3}, {6,2})]);  // 2-D slice from the 3-D cube
cout <<= r3;                                    // Output: 1 3 5 7 9 11
valarray<char> text("now it really starts", 20);
valarray<char> caps("NIRS", 4);
valarray<size_t> idx{ 0, 4, 7, 14 };            // Indexes in text
text[idx] = caps;                               // assign indirectly
cout <<= text;                                  // Output: Now It Really Starts