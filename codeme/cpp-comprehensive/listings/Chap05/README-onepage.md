# Listings of Chap05.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## GodboltId:eP77v498E

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/eP77v498E 
#include <iostream> 
int func(int arg1, int arg2) { 
    if(arg1 > arg2) { 
        return arg1-arg2; 
    } else { 
        return arg2-arg1; 
    } 
} 
int main(int argc, const char* argv[]) { 
    for(int x=0; x<10; ++x) { 
        for(int y=0; y<10; ++y) { 
            std::cout << func(x,y) << " "; 
        } 
        std::cout << "\n"; 
    }
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/eP77v498E](https://godbolt.org/z/eP77v498E):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eP77v498E 
#include <iostream> 
int func(int arg1, int arg2) { 
    if(arg1 > arg2) { 
        return arg1-arg2; 
    } else { 
        return arg2-arg1; 
    } 
} 
int main(int argc, const char* argv[]) { 
    for(int x=0; x<10; ++x) { 
        for(int y=0; y<10; ++y) { 
            std::cout << func(x,y) << " "; 
        } 
        std::cout << "\n"; 
    }
}
```

----------------


## GodboltId:4TMn37qhE

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/4TMn37qhE](https://godbolt.org/z/4TMn37qhE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
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
```

----------------

