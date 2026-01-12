
// https://godbolt.org/z/jaMTcshEf 
#include <iostream>
#include <string>

using std::string; using std::cout;

void guessMonth(unsigned whatDayIsItToday) {
    switch(whatDayIsItToday) {
    /* missing break statements: fall-through intended */
    default:
        if(whatDayIsItToday>31) {
            cout << "You are cheating";
            break;
        }
    case 28:
    case 29:
        cout << "Feb ";
    case 30:
        cout << "Apr Jun Sep Nov ";
    case 31:
        cout << "Jan Mar May Jul Aug Oct Dec ";
    }

    cout << ". ";
}
int main() {
    guessMonth(31);  // if today were the 31st?
    // Output: Jan Mar May Jul Aug Oct Dec .
    guessMonth(30);  // what if it were the 30th?
    // Output: Apr Jun Sep Nov Jan Mar May Jul Aug Oct Dec .
    guessMonth(4);
    // Output: Feb Apr Jun Sep Nov Jan Mar May Jul Aug Oct Dec .
    guessMonth(77);
    // Output: You are cheating.
}