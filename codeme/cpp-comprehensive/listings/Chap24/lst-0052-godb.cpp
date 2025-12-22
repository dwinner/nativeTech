//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fe8bev9Y9 
std::vector v{ 1, 2, 3, 4, 5, 6 };
v.erase(v.begin()+2, v.begin()+5);  // v is now {1, 2, 6}
v.erase(v.begin(), v.end());        // deletes the rest