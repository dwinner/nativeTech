//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6rG393Wa5
vector<int> primes{ 2,3,5,7,11 };
vector evens{ 2,4,6,8,10 };
vector<int> notLikeThis{ 'a', 4.3, 8L }; // (ERR) "Narrowing" double not okay
vector<string> names{ "are", "only" };   // converts arguments
vector sound{ "smoke", "fume" };         // dangerous: vector<const char[]>
vector wet{ "rain"s, "water"s };         // vector<string>
vector cold{ "ice"sv, "pole"sv };        // vector<string_view>