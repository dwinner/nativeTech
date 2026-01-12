
// https://godbolt.org/z/xYvPxbjr5 
#include <iostream>                    // cin, cout for input and output 
#include <string> 
#include <array> 
using std::cin; using std::cout;       // abbreviations cin and cout 
void input( 
    std::string &name, 
    unsigned &birthDay, 
    unsigned &birthMonth, 
    unsigned &birthYear, 
    long long &socialSecurity, 
    std::array<int,12> &monthIncomes)  // array is a container 
{ 
    /* input still without good error handling... */ 
    cout << "Name: "; 
    std::getline(cin, name);           // getline takes input stream and string 
    if(name.length() == 0) { 
        cout << "You have entered an empty name.\n"; 
        exit(1); 
    } 
    cout << "birth day: "; cin >> birthDay; 
    cout << "birth month: "; cin >> birthMonth; 
    cout << "birth year: "; cin >> birthYear; 
    cout << "social security: "; cin >> socialSecurity; 
    for(int m=0; m<12; ++m) { 
        cout << "income month " << m+1 << ": "; // multiple outputs
        cin >> monthIncomes[m];                 // read in with operator 
    } 
    cout << std::endl; 
} 
int main() { 
    std::string name{}; 
    unsigned day = 0; 
    unsigned month = 0; 
    unsigned year = 0; 
    long long ssid = 0; 
    std::array<int,12> incomes{};
    input(name, day, month, year, ssid, incomes); 
    // ... calculations ... 
}