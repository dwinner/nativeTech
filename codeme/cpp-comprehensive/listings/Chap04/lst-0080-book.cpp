
// https://godbolt.org/z/We3oWrcWW 
#include <iostream> 
#include <cmath>     // sqrt 
#include <concepts>  // floating_point 
using std::min; using std::max; using std::floating_point;

template<floating_point T> T heron(T a, T b, T c) { 
    auto s = (a+b+c) / 2; 
    return sqrt(s*(s-a)*(s-b)*(s-c)); 
}

template<floating_point T> T kahan(T a, T b, T c) { 
    auto x = max(a,max(b,c)); 
    auto y = max(min(a,b), min(max(a,b),c)); 
    auto z = min(a,min(b,c)); return 
    sqrt( (x+(y+z))*(z-(x-y))*(z+(x-y))*(x+(y-z)) )/4 ; 
}

template<floating_point T> void triangle(T a, T b, T c) { 
    std::cout << "heron: " << heron(a,b,c) << '\n'; 
    std::cout << "kahan: " << kahan(a,b,c) << '\n'; 
}

int main() { 
    triangle(3.0f, 4.0f, 5.0f); 
}