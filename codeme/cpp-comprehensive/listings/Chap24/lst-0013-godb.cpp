//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TG1qd31Mq 
using namespace std; namespace views = std::ranges::views;
vector in { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
auto out = in
    | views::filter([](int i) { return i%3 == 0; })
    | views::transform([](int i) {return i*i; });