//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/9jGKPbzqo
struct MyValue { /* something */ };
MyValue globalValue{};                   // global class instance

void function(const MyValue &paramRef) {
    if( /*...*/ ) function( /*x1?*/ );   // call some function
    MyValue localValue{};                // local class instance
}                                        // end of function

int main() {
    MyValue mvalue1{};
    function( /*x2?*/ );
    function( MyValue{} );               // temporary value
    {
        MyValue mvalue2{};
        function( /*x3?*/ );
        MyValue mvalue3{};
    }                                    // end of inner block
    function( /*x4?*/ );
    MyValue mvalue4{};
    function( /*x5?*/ );
}                                        // end of main function