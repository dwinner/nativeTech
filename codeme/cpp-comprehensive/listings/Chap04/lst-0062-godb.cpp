//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xrMYz93WP 
int main() { 
    for(int w1 = 1; w1 <= 6; ++w1) { // 1..6 
        for(int w2 = 0; w2 < 10; ++w2) { // 0..9 
            int max = w1 > w2 ? w1 : w2; // ternary operator 
        }
    }
}