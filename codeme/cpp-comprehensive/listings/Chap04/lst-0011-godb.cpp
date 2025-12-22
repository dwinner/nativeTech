//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/33cc3xfo6 
#         include <iostream>       // # must be at the beginning of the line 
int            main( 
    ){ 
    std::cout 
<<"This is " 
        "text with <brackets>\n"   // string literal interrupted by new line 
        ; 
        /*type:*/ int 
        /*Variable:*/ a_Value 
        /*Init:*/ = 100;           // inner comments 
std::cout<<a_Value<<"\n";}         // no spaces