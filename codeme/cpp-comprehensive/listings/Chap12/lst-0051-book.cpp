
// https://godbolt.org/z/T7afh4vxz 
#include <vector>
std::vector<int> createData(unsigned size) {
    std::vector<int> result{};
    for(int idx=0; idx<size; ++idx) {
        result.push_back(idx);
    }
    return result;
}