
// https://godbolt.org/z/d6PscKx4M 
#include <map>
#include <string>
#include <algorithm> // any_of
#include <iostream>  // cerr
std::map<int, std::string> m;
int main() {
    if(auto it = m.find(10); it != m.end()) { return it->second.size(); }
    if(char buf[10]={0}; std::fgets(buf, 10, stdin)) { m[0] += buf; }
    std::string s;
    if(auto keywords = {"if", "for", "while"};
        std::any_of(keywords.begin(), keywords.end(),
        [&s](const char* kw) { return s == kw; })) {
            std::cerr << "Error ";
   }
}