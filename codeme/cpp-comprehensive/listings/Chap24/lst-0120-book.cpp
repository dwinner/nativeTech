
// https://godbolt.org/z/6MKMj3Pxv
#include <stack>
void run(auto data) { /* ... */ }  // C++20, abbreviated function template
run(stack<int>{});                 // Default: uses vector<int>
run(stack<int,vector<int>>{});     // like the default
run(stack<int,list<int>>{});       // uses list<int>