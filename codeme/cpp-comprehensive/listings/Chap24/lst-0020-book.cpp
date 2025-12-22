
// https://godbolt.org/z/3dYhh6n5f
#include <set>
#include <vector>
#include <iostream>
template<class T> class GobAllocator  {
public:
    using value_type = T;
    T* allocate(size_t count) {
        size_t add = sizeof(T)*count;
        std::cout << "allocate("<<add<<"/"<<(buf_.size()-current_)<<")\n";
        if(current_+add > buf_.size()) throw std::bad_alloc{};
        char* result = buf_.data()+current_;
        current_ += add;
        return reinterpret_cast<T*>(result);
    }
    void deallocate(T* p, size_t count) {
        size_t del = sizeof(T)*count;
        std::cout << "deallocate("<<del<<")\n";
        if(del==current_ && p==reinterpret_cast<T*>(buf_.data())) {
            std::cout << "...all free.\n";
            current_ = 0;                // release everything again
        }
    }
    GobAllocator() : GobAllocator{1024} {}
    explicit GobAllocator(size_t mx)
      : buf_(mx, 0), current_{0} { }
private:
       std::vector<char> buf_;
       size_t current_;
};
int main() {
    constexpr size_t CNT = 1*1000*1000;
    using Gob = GobAllocator<int>;
    try {
        Gob gob(CNT*sizeof(int));        // prepare allocator
        std::vector<int,Gob> data(gob);
        data.reserve(CNT);               // get memory in one go
        for(int val=0; val < (int)CNT; ++val)
            data.push_back(val);
    } catch(std::bad_alloc &ex) {
        std::cout << "Memory exhausted.\n";
    }
}