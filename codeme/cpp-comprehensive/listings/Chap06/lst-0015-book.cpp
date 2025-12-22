
// https://godbolt.org/z/cd1d1W7T1 
#include <array> 
#include <iostream> 
using std::cout; using std::array; using std::string; 
int main() {
    array<string,7> wday = { "Monday", "Tuesday",         // define 
        "Wednesday", "Thursday", "Friday", "Saturday", "Sunday" }; 
    cout << "The week starts with " << wday[0] << ".\n";  // read values
    cout << "It ends with " << wday.at(6) << ".\n";       // read safely 
    /* nothern? */
    wday[5] = "Satdee";                                   // change values 
}