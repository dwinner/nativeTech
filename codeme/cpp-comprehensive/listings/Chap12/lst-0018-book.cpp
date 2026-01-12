
// https://godbolt.org/z/bz6n3Pjrv 
// Excerpt. Person as before
void Person::print(std::ostream& os) {
    os << format("{} ({}) from {}", name_, age_, city_);
}
int main() {
    Person carl {"Carl", 12, "Toledo"};
    carl.print(cout);          // on the screen
    cout << "\n";
    std::ofstream file {"persons.txt"};
    carl.print(file);          // to a file
    // automatic test:
    std::ostringstream oss{};  // writes to a string
    carl.print(oss);
    if(oss.str() == "Carl (12) from Toledo") {
        cout << "ok\n";
    } else {
        cout << "Error in Person::print!\n";
        return 1;              // propagate error outward
    }
}