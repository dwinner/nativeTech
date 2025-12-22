//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8TdzEGf3d 
void mainProgram() {
    std::jthread th{ &task1 };
    std::vector data{ 0,1,2 };
    data.at(666);                  // (ERR) triggers out_of_range
}