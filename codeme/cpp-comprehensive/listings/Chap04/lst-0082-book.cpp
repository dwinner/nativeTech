
// https://godbolt.org/z/sEE94bh5K
#include <iostream>
#include <iomanip> // setprecision, fixed 
constexpr int framesPerSec = 25; 
constexpr int runtimeInSecs = 3600; 
constexpr int framesTotal = runtimeInSecs * framesPerSec; 
constexpr float frametime = 1.0f / framesPerSec;

int main() { 
    float filmtime = 0.f; 
    for(int n=1; n <= framesTotal; ++n) { // 1 .. framesTotal
        filmtime += frametime;            // accumulate 
        // ... here code for this frame ...
    } 
    std::cout << std::setprecision(10) << std::fixed 
        << filmtime << '\n';              // output: 3602.2695312500 
}