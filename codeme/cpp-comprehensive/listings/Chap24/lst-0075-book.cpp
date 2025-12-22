
// https://godbolt.org/z/oc9xxev95
// without arguments
set<int> empty{};
cout <<= empty;          // Output:
// initializer list
set list{ 1,1,2,2,3,3,4,4,5,5 };  // set does not take duplicates
cout <<= list;           // Output: 1 2 3 4 5
// copy
set copy(list);
cout <<= copy;           // Output: 1 2 3 4 5
// pair of iterators
set from_to( std::next(list.begin()), std::prev(list.end()));
cout <<= from_to;        // Output: 2 3 4
// Range
set even(from_range, list | vs::filter([](int i){ return i%2; }));
cout <<= from_to;        // Output: 2 4