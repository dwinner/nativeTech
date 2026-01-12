//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/MszTjE3dv 
const regex pattern{R"(^score)"
    R"(\s+)"
    R"((\d+))"          // points
    R"(\s+)"
    R"(for)"
    R"(\s+)"
    R"((\d+))"          // number of nights
    R"(\s+)"
    R"(night)"
    R"(s?)"             // optional: plural
    R"(\s+)"
    R"(at)"
    R"(\s+)"
    R"((.*))"           // hotel name
    R"()"};