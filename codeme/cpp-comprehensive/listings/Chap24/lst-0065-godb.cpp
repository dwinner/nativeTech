//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9b9fY4Mve 
#include <iostream>
#include <forward_list>
int main()     {
    std::forward_list mylist = {20, 30, 40, 50};
    mylist.insert_after(mylist.before_begin(), 11 );
    for (int& x: mylist) std::cout << ' ' << x; // Output: 11 20 30 40 50
    std::cout << '\n';
}