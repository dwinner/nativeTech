//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y8KK3nEG3
void sortData(std::ranges::random_access_range auto &&data) {
    std::ranges::sort(data);
}