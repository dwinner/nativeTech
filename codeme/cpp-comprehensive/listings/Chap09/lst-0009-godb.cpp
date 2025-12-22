//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/K9c913had 
if(int result; (result = read(buffer, 100)) != 0) {
    std::cerr << "Error number "<< result << " occurred.\n";
}