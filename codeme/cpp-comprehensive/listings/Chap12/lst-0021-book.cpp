
// https://godbolt.org/z/a7q5xsWGj 
// Excerpt …
    std::ostream& print(std::ostream& os);
};
std::ostream& Person::print(std::ostream& os) {
    return os << format("{} ({}) from {}", name_, age_, city_);
}
std::ostream& operator<<(std::ostream& os, Person p) {
    return p.print(os);
}
int main() {
    Person paul {"Paul", 23, "Irvine"};
    cout << "You are " << paul << ", right?\n";
}