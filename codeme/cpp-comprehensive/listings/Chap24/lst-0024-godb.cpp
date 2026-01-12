//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7zMP5fo7c 
vector<int> dataA;
vector<int> dataB{};
vector<int> dataC = {};   // no assignment
cout << format("{} {} {}\n", dataA.size(), dataB.size(), dataC.size()); // 0 0 0