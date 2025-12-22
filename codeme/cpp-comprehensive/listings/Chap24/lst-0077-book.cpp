
// https://godbolt.org/z/8eGvWY45o 
set source{1,2,3,4,5};
set<int> target{};
set<int> target2{};
cout <<= source;         // Output: 1 2 3 4 5
cout <<= target;         // Output:
cout <<= target2;        // Output:
target = source;         // copy afterward
cout <<= source;         // Output: 1 2 3 4 5
cout <<= target;         // Output: 1 2 3 4 5
target2 = std::move(source); // move
cout <<= source;         // Output:
cout <<= target2;        // Output: 1 2 3 4 5