
// https://godbolt.org/z/8TdzEGf3d 
void mainProgram() {
    std::jthread th{ &task1 };
    std::vector data{ 0,1,2 };
    data.at(666);                  // (ERR) triggers out_of_range
}