//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/GbbcnYbqv 
std::vector vec { 1, 2, 3 };        // instead of vector<int>
std::tuple tpl { 5, 'x' };          // instead of tuple<int,char> 
std::shared_ptr<int> ptr { new int(5) };
std::shared_ptr ptr2 { ptr };       // instead of shared_ptr<int>