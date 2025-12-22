
// https://godbolt.org/z/6hh5KhExM 
#include <iostream>                // cin
#include <string>
int main() {
    std::string line;
    do {                             // execute getline at least once
       std::getline(std::cin, line);
       if(!std::cin) break;          // error or end of file
    } while(line != "quit");         // end on specific input
}