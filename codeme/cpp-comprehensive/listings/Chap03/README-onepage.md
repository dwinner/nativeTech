# Listings of Chap03.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 03.2 C++: In C++, you have values instead of references.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
Data data{5}; 
Data more{6}; 
data = more;
more.value = 7; 
cout << data.value << '\n'; // still 6
```

----------------


## Listing 03.3 C++: Modern C++ with heap memory uses auxiliary classes such as shared_ptr.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/WM11Gs6sz 
auto data = make_shared<Data>(5); 
auto more = make_shared<Data>(6); 
data = more;
more->value = 7; 
cout << data->value << '\n'; // now also 7
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/WM11Gs6sz](https://godbolt.org/z/WM11Gs6sz):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/WM11Gs6sz 
auto data = make_shared<Data>(5); 
auto more = make_shared<Data>(6); 
data = more;
more->value = 7; 
cout << data->value << '\n'; // now also 7
```

----------------


## Listing 03.4 C++: Without modern C++ tools, special attention must be paid to pointer ownership.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/Ee7eqq1MY 
Data* dataOwner = new Data(5); 
Data* data = dataOwner; 
Data* more = new Data(6); 
data = more;
more->value = 7; 
cout << data->value << '\n'; // now also 7 
/* tidy up */ 
delete more; 
delete dataOwner;
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/Ee7eqq1MY](https://godbolt.org/z/Ee7eqq1MY):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Ee7eqq1MY 
Data* dataOwner = new Data(5); 
Data* data = dataOwner; 
Data* more = new Data(6); 
data = more;
more->value = 7; 
cout << data->value << '\n'; // now also 7 
/* tidy up */ 
delete more; 
delete dataOwner;
```

----------------


## Listing 03.6 C++: const can protect the value or the reference.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/Moe5M7sK8 
Data const * data = new Data(5);
data->value = 7;          // this const protects Data 
data = new Data(6);       // reassigning pointer is okay 
Data * const more = new Data(8); 
more->value = 9;          // now okay
more = new Data(10);      // reference is protected
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/Moe5M7sK8](https://godbolt.org/z/Moe5M7sK8):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Moe5M7sK8 
Data const * data = new Data(5);
data->value = 7;          // this const protects Data 
data = new Data(6);       // reassigning pointer is okay 
Data * const more = new Data(8); 
more->value = 9;          // now okay
more = new Data(10);      // reference is protected
```

----------------

