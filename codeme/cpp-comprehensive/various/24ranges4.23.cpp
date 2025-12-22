//    #!cpp filename=24ranges4.cpp gboptions="-std=c++23" testcxxflags="-std=c++23"
    #include <vector> //#
    #include <ranges> //#
    #include <iostream> //#
    #include <algorithm> // ranges::for_each
    // ... as before ...
    int main() { //#
    using namespace std; namespace views = std::ranges::views; //#
    vector in{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }; //#
    auto out = in
        | views::filter([](const int i) { return i%3 == 0; })
        | views::transform([](const int i) {return i*i; })
        | std::ranges::to<vector>;
    for(const auto i : out) //#
        cout << i << ' '; //#
    cout << '\n'; //#
    //= 0 9 36 81
    } //#

