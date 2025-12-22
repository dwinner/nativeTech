    #include <set>
    #include <vector>
    #include <iostream>
    template<class T> class ChunkAllocator  {
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
                std::cout << "...all freed.\n";
                current_ = 0;// free everything
            }
        }
        ChunkAllocator() : ChunkAllocator{1024} {}
        explicit ChunkAllocator(size_t mx)
          : buf_(mx, 0), current_{0} { }
    private:
           std::vector<char> buf_;
           size_t current_;
    };
    int main() {
        constexpr size_t AMOUNT = 1*1000*1000;
        try {
            ChunkAllocator<int> chunk(AMOUNT*sizeof(int));
            std::vector<int,ChunkAllocator<int>> data(chunk);
            data.reserve(AMOUNT);
            //= allocate(4000000/4000000)
            for(int val=0; val < (int)AMOUNT; ++val)
                data.push_back(val);
        } catch(std::bad_alloc ex) {
            std::cout << "Out of memory.\n";
        }
        //= deallocate(4000000)
        //= ...all freed.
    }
