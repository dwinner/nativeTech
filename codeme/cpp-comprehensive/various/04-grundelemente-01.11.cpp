//    #!cpp filename=04-grundelemente-01.cpp testargs=96
#include <iostream>                                // for std::cin, std::cout, std::endl
#include <string>                                  // std::stoi

void calculate(int n) {                            // your own function
    using namespace std;                           // for std::cout and std::endl
    /* print factors */
    cout << "Factors of " << n << " are:\n";
    if(n == 0) { cout << "0\n"; return; }          // 0 is a factor of 0
    for(int factor=1; factor <= n; ++factor) {     // instead of factor=factor+1
        if(n % factor == 0)
            cout << factor << ", ";
    }
    cout << endl;
}

int main(int argc, const char* argv[]) {           // arguments for main
    calculate(0);                                  // function call
    return 0;
}