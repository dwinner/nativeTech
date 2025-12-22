
template <auto value> void f() { }
f<10>();                // value gets type int
f<'a'>();               // value gets type char