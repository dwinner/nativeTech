//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jer44aWs9 
auto create(string name, int age, string city) {
    return Person{name, age, city};  // auto requires constructor name
}
auto create2(string name, int age, string city) {
    return {name, age, city};        // (ERR) auto with initializer_list does not work
}