//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/vcb9qP8jK
#include <vector>
#include <thread>
#include <mutex>
#include <iostream>
#include <numeric>  // iota
#include <concepts> // copyable 

/* T: noexcept copyable and assignable */
template<std::copyable T>
class MxStack {
    std::vector<T> data_;
    std::mutex mx_;

public:
    MxStack() : data_{} {}

    bool isEmpty() const { return data_.empty(); }

    void push(const T& val) {
        std::lock_guard<std::mutex> g{mx_};
        data_.push_back(val);
    }

    T pop() {
        std::lock_guard g{mx_};
        if(data_.empty())
            throw std::length_error{"empty stack"};
        T tmp{std::move(data_.back())};
        data_.pop_back();
        return tmp;
    }
};

int main() {
    // Prepare stack
    MxStack<int> mxs{};
    for(int i=1; i<=1'000'000; ++i) mxs.push(i);
    // Define computation
    auto sumIt = [&mxs](long &sum) {
        int val{};
        try {
            while( ! mxs.isEmpty()) {
                sum += mxs.pop(); // might still throw
            }
        } catch(std::length_error &ex) {}
    };
    // Compute
    long sum1 = 0;           // for partial result
    std::jthread th1{sumIt, std::ref(sum1)};
    long sum2 = 0;           // for partial result
    std::thread th2{sumIt, std::ref(sum2)};
    th1.join(); th2.join();
    long sum = sum1 + sum2; // Total result
    // Result
    std::cout << "Expected result: "
        << (1'000'000L*1'000'001)/2 << '\n'; // Output: 500000500000
    std::cout << "Actual: "
        << sum << '\n';                      // Output: 500000500000
}