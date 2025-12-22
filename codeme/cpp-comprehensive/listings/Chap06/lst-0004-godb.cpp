//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1zfo7fPcs
#include <iostream> 
#include <string>
#include <string_view> 
void show_middle(std::string_view msg) {       // string_view is a good parameter
    auto middle = msg.substr(2, msg.size()-4); // substr returns string_view 
    std::cout << middle << "\n"; 
} 
int main() { 
    using namespace std::literals; 
    const std::string aaa = "##Some text##"s; 
    show_middle(aaa);                          // conversion to string_view 
    auto bbb = "++More text++"sv;              // string_view as literal 
    show_middle(bbb); 
}