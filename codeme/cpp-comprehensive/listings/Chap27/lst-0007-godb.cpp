//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/exKMv8646
#include <fstream>
#include <iostream>
using std::cout; using std::cin; using std::ofstream;
void checkIOstate(std::ios& stream) {
    if( stream.good() ) {
        cout << "All good\n";
    } else if( stream.bad() ) {
        cout << "Fatal error\n";
    } else if( stream.fail()) {
        cout << "I/O error\n";
        if( stream.eof()) {
            cout << "End of stream reached\n";
        }
    }
    stream.clear();
}
int main() {
    int val=0;
    cout << "Enter value: ";
    cin >> val;
    checkIOstate( cin );
    std::ifstream file;
    file.open("nonexistent.text");
    checkIOstate(file);
    std::fstream fstr;
    fstr.open("newFile.txt",
        ofstream::out | ofstream::in
        | ofstream::binary | ofstream::trunc);
    fstr << "Text in the file\n";
    fstr.seekp(std::ios_base::beg);
    char ch;
    while( fstr.good()) {
        fstr.get(ch);
        if(fstr.good()) cout.put(ch);
    }
    checkIOstate(fstr);
}