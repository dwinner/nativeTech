//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/baPeoTbM8 
template<>   // full specialization
class MyPair<std::string, std::string> {
    std::vector<std::string> data_;
public:
    MyPair(const std::string& t, const std::string& u) : data_{t, u} { }
};
int main() {
    // uses the full specialization:
    MyPair<std::string,std::string> strStr{"c","d"};
}