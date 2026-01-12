//%%lu: This prevents a template function from being called with pointers.
//
// Example does not work! Still compiles!
//
//    #!cpp filename=23ConceptsNoPointer.cpp
    #include <iostream>
    #include <concepts>

    auto bad_max_value(auto a, auto b)
    { return a > b ? a : b; }

    auto good_max_value(auto a, auto b)
      requires std::totally_ordered_with<decltype(a), decltype(b)> &&
      requires { std::is_pointer_v<decltype(a)> == false &&
                 std::is_pointer_v<decltype(b)> == false; }
    { return a > b ? a : b; }

    int main() {
        int a = 1, b = 2;
        std::cout << bad_max_value(&a, &b) << '\n';  //@ERR@ Pointer comparison not desired
        //= ???
        #if 1 //#
        std::cout << good_max_value(&a, &b) << '\n';  // Compiler error, as desired
        #endif //#
        std::cout << good_max_value(a, b) << '\n';  // (+Output:+) 2
        //= 2
    }
