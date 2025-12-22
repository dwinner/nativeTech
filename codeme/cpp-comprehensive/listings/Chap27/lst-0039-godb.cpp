//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8hK9Y4vhd
#include <fstream>
#include <iostream>
#include <memory> // unique_ptr
int main() {
    std::fstream file("27Streams.tex");          // Open file for reading
    auto bufptr = file.rdbuf();                  // std::streambuf*
    auto size = bufptr->pubseekoff(0, file.end); // std::streamsize
    bufptr->pubseekoff(0, file.beg);             // back to the beginning
    auto buffer = std::unique_ptr<char[]>(new char[size]); // allocate memory
    auto n = bufptr->sgetn(buffer.get(), size);  // transfer number of chars
    std::cout << "Characters read: " << n << "\n";
    std::cout.write(buffer.get(), size);         // Output char[]
}