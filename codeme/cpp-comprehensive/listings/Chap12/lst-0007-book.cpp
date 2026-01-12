
// https://godbolt.org/z/ME76bfbWa 
Person create(string name, int age, string city) {
    return Person{name, age, city};       // returned directly
}
int main() {
    print(create("John", 45, "Boston"));  // return value used directly
}