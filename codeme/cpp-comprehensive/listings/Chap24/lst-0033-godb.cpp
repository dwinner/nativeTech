//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jjWx3d375
vector d{ 1, 2, 4, -1, 1, 2, -2 };
for(size_t idx=0; idx < d.size(); ) {  // checks vector boundary
    cout << d[idx] << ' ';             // additional check with at not necessary
    idx += d[idx];                     // same here
}
cout << '\n';
// Output: 1 2 -1 4 -2 1 2