//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fjc37rvEr 
int add(int n=0, int m=0, int o=0, int p=0, int q=0)  {
    return n+m+o+p+q;
}
int main() {
std::cout << add(1,2,3,4,5) << " ";
std::cout << add(1,2,3,4) << " "; // like add(1,2,3,4,0)
std::cout << add(1,2,3) << " ";   // like add(1,2,3,0,0)
std::cout << add(1,2) << " ";     // like add(1,2,0,0,0)
std::cout << add(1) << " ";       // like add(1,0,0,0,0)
std::cout << add() << " ";        // like add(0,0,0,0,0)
}