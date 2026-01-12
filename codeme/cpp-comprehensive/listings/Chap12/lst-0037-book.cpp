
ostream& operator<<(ostream& os, Person p) {
    return os << format("{} ({}) from {}", p.name_, p.age_, p.city_);
}