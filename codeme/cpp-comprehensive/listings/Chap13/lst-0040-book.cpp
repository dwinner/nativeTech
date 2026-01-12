
struct Data {
    static constexpr size_t LATE; // (ERR) does not work without direct initialization
    static constexpr size_t EARLY = 10;
};
constexpr size_t Data::LATE = 10; // (ERR) with constexpr, definition is different