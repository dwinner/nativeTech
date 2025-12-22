
// https://godbolt.org/z/cveT8Ej7q
struct EqEntry {
    bool operator()(const Entry&a, const Entry&b) const {
        return a.city_==b.city_;
    }
};

struct HashEntry {
    std::hash<string> sHash;
    std::hash<int> iHash;
    size_t operator()(const Entry& a) const {
        return sHash(a.city_) ^ iHash(a.zip_); // (ERR) too many elements
    }
};