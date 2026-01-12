//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/arfee3TTP 
using namespace std;
vector in { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
vector<int> tmp{};
vector<int> out{};
copy_if(in.begin(), in.end(), back_inserter(tmp), [](int i) { return i%3 == 0; });
transform(tmp.begin(), tmp.end(), back_inserter(out), [](int i) {return i*i; });