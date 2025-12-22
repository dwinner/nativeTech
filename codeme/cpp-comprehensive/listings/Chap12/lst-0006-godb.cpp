//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rrjPorT1o
// Snippet
Person create(string name, int age, string city) { // return type
    Person result {name, age, city};
    return result;
}
int main() {
    Person john = create("John", 45, "Boston");    // store return value
    print(john);
}