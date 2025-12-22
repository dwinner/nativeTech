
// https://godbolt.org/z/MPPsbdq9c 
#include <iostream> // cin, cout for input and output

void input(unsigned &birthDay_, 
           unsigned &birthMonth_, 
           unsigned &birthYear_, 
           unsigned long long &taxnumber_, 
           double &bodyheight_)  
{ 
    /* Inputs still without good error handling... */ 
    std::cout << "Date of birth: "; std::cin >> birthDay_; 
    std::cout << "Month of birth: "; std::cin >> birthMonth_; 
    std::cout << "Year of birth: "; std::cin >> birthYear_; 
    std::cout << "Tax number: "; std::cin >> taxnumber_; 
    std::cout << "Height: "; std::cin >> bodyheight_; 
} 

int main() { 
    /* data */ 
    unsigned birthDay_ = 0;
    unsigned birthMonth_ = 0; 
    unsigned birthYear_ = 0; 
    unsigned long long taxnumber_ = 0; 
    double bodyheight_ = 0.0; 
    /* Input */ 
    input(birthDay_, birthMonth_, birthYear_, taxnumber_, bodyheight_); 
    /* Calculations */ 
    // ... 
}