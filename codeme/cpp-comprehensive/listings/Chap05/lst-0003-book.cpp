
// https://godbolt.org/z/4TMn37qhE 
#include <iostream> 
int func ( int arg1, int arg2 ) 
{ 
  if (arg1 > arg2) 
    return arg1 - arg2; 
  else 
    return arg2 - arg1; 
} 
int main ( int argc, const char *argv[] ) 
{ 
  for ( int x = 0 ; x < 10 ; ++x ) 
  { 
    for ( int y = 0 ; y < 10 ; ++y ) 
      std::cout << func ( x, y ) << " "; 
    std::cout << "\n"; 
  }
}