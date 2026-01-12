
// https://godbolt.org/z/zMqjqMeav
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    year y{2021};
    std::cout << y << "\n";
    month m{October};
    auto result = m + months{3};   // 'months', not 'month': 3 months later
    std::cout << result << "\n";   // overflowed to January
    weekday wd{Thursday};
    auto result = wd + days{4};    // 'days', not 'day': 4 days later
    std::cout << result << "\n";   // overflowed to Monday
    weekday sun1{0};               // 0 is Sunday
    weekday sun2{7};               // 7 is also Sunday
    std::cout << sun1 << "\t" << sun2 << "\n";
    weekday_indexed wdi{wd, 4};    // unspecified 4th Thursday
    std::cout << wdi << "\n";      // Output: Thu[4] — as given by chrono 
                                   // unspecified values
}