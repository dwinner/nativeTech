//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zWa58dYYq 
#include <map>
#include <iostream>
using std::map; using std::cout;
int main() {
    map<int,char> alpha;
    cout << alpha.size() << '\n';        // 0 naturally
    if( alpha[5] == '3' ) { /* ... */ }
    cout << alpha.size() << '\n';        // now 1
    char x = alpha[99];                  // works
    cout << alpha.size() << '\n';        // and now 2
}