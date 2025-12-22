
// https://godbolt.org/z/M6en1GMrE 
#include <iostream>            // cout
#include <string>
void print(int n)  {           // function print for type int
    std::cout << "Number:" << n << "\n";
}
void print(std::string s)  {   // same name, different type
    std::cout << "String:" << s << "\n";
}
int main() {
    int number = 10;
    std::string name = "Bilbo";
    print(number);             // calls print(int), number is int
    print(name);               // calls print(string), name is string
    print(11 + 22);            // expression is int
    print(name + " Baggins");  // expression is string
}