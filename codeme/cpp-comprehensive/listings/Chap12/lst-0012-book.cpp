
struct Person {
    //… rest as before …
    string greeting();
};
string Person::greeting() {
    return format("Hello {} from {}", this->name_, this->city_);
}
int main() {
    Person anna { "Anna", 33, "Eek" };
    Person nina { "Nina", 22, "Ojo" };
    anna.greeting();
    nina.greeting();
}