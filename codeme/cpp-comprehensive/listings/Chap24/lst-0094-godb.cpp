//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8948besY8
map<string,string> data { {"John","Wayne"}, {"Cary","Grant" }, };
cout <<= data;                         // Cary:Grant John:Wayne
data.rbegin()->second = "Travolta";    // Overwrite target
cout <<= data;                         // Cary:Grant John:Travolta