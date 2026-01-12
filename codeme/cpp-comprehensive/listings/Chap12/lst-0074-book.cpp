
// https://godbolt.org/z/EYjo8E8Gn 
class Year {
public:
    using value_type = int;            // introduce type alias
    value_type value_;                 // actually internally used type
public:
    explicit Year(value_type v) : value_{v} {}
    value_type value() { return value_; }
};
int main() {
   Year year{ 2024 };                   // rely on compiler conversion here
   Year::value_type val = year.value(); // use ::
}