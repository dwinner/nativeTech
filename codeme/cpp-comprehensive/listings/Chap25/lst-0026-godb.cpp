//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/n8ErWj7j6 
template<typename It, typename Func>
void adjacent_pair(It begin, It end, Func func) {
    if(begin != end) {
        It prev = begin;    // first argument
        ++begin;            // second argument
        for(; begin != end; ++begin, ++prev) {
            func(*prev, *begin);
        }
    }
}