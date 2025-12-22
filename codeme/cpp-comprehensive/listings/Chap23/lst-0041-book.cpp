
struct ByYear { // implements less-than by Dwarf::year_
    bool operator()(const Dwarf& a, const Dwarf& b) const {
        return a.year_ < b.year_;
    }
};