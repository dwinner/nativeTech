
std::ostream& Person::print(std::ostream& os) {
    return os << format("{} ({}) from {}", name_, age_, city_);
}
std::ostream& operator<<(std::ostream& os, Person p) {
    return p.print(os);
}