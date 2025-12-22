# Listings of Chap30.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## GodboltId:veEq4dcET

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/veEq4dcET 
struct Day {
    Day(int a, int b) : Day{} {       // delegates
        if(a==0 || b == 0)
            throw 666;                // throws exception
    }
    Day() {}
};
int main() {
    try {
        Day day{1,2};
    } catch(int) { }
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/veEq4dcET](https://godbolt.org/z/veEq4dcET):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/veEq4dcET 
struct Day {
    Day(int a, int b) : Day{} {       // delegates
        if(a==0 || b == 0)
            throw 666;                // throws exception
    }
    Day() {}
};
int main() {
    try {
        Day day{1,2};
    } catch(int) { }
}
```

----------------


## GodboltId:Y8KK3nEG3

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/Y8KK3nEG3
void sortData(std::ranges::random_access_range auto &&data) {
    std::ranges::sort(data);
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/Y8KK3nEG3](https://godbolt.org/z/Y8KK3nEG3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y8KK3nEG3
void sortData(std::ranges::random_access_range auto &&data) {
    std::ranges::sort(data);
}
```

----------------

