//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/PebG9bn9K
set<Dwarf,ByYear> dwarves2{begin(dwarves), end(dwarves)};
for(const auto& z : dwarves2) // differently sorted output
    cout << z.year_ << " ";