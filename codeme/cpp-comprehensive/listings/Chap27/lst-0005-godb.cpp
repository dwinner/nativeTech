//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/MaEzcccTG
#include <iostream>
using std::cout; using std::cin; using std::endl;
int main() {
    const unsigned int MAX = 10;
    char buffer[MAX] = {0};
    cout << "Input getline : ";
    cin.getline(buffer, MAX);
    cout << std::cin.gcount()
        << " characters were read\n";
    for(auto c : buffer) {
        if(c && c!='\0') cout.put(c);
    }
    cin.ignore(MAX, '\n');
    cout << "\nMake input (end with .) : ";
    char ch=0;
    while(cin.get(ch)) {
        if(ch == '.') break;
        cout.put(ch);
    }
    cout << "Input ended" << endl;
}