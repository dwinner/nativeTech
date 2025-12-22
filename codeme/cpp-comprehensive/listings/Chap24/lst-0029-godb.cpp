//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/addobjfsK 
vector<int> zeros(10);        // 10 zeros
vector<int> sixes(10, 6);     // 10 sixes
vector<int> ten{10};          // (ERR) Attention! Only one 10
vector<int> tenSix{10, 6};    // (ERR) Attention! Two elements 10 and 6