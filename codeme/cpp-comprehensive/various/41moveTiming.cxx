#include <string>
#include <vector>
#include <chrono> // time measurement
#include <memory> // unique_ptr
#include <iostream> // cout

using std::vector; using std::cout;

using namespace std::chrono;

long long millisSeit(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}

struct RuleOfZero {
    std::unique_ptr<int> val_;
    RuleOfZero(int val) : val_{new int{val}} {}
};

struct DelMove {
    std::unique_ptr<int> val_;
    DelMove(int val) : val_{new int{val}} {}
    ~DelMove() {};
    DelMove(const DelMove& o) noexcept : val_{new int{*o.val_.get()}} {};
    DelMove& operator=(const DelMove& o) noexcept { val_.reset(new int{*o.val_.get()}); return *this; }
    DelMove(DelMove&&) = delete;
    DelMove& operator=(DelMove&&) = delete;
};

struct MoveExcept {
    std::unique_ptr<int> val_;
    MoveExcept(int val) : val_{new int{val}} {}
    ~MoveExcept() {};
    MoveExcept(const MoveExcept& o) /*noexcept*/ : val_{new int{*o.val_.get()}} {};
    MoveExcept& operator=(const MoveExcept& o) /*noexcept*/ { val_.reset(new int{*o.val_.get()}); return *this; }
    MoveExcept(MoveExcept&&) = delete;
    MoveExcept& operator=(MoveExcept&&) = delete;
};

struct MoveNoExcept {
    std::unique_ptr<int> val_;
    MoveNoExcept(int val) : val_{new int{val}} {}
    ~MoveNoExcept() {};
    MoveNoExcept(const MoveNoExcept& o) noexcept : val_{new int{*o.val_.get()}} {};
    MoveNoExcept& operator=(const MoveNoExcept& o) noexcept { val_.reset(new int{*o.val_.get()}); return *this; }
    MoveNoExcept(MoveNoExcept&&) = delete;
    MoveNoExcept& operator=(MoveNoExcept&&) = delete;
};

struct RuleOfZeroShared { // really shares data on copy
    std::shared_ptr<int> val_;
    RuleOfZeroShared(int val) : val_{new int{val}} {}
};
struct MoveExceptShared { // really copies all data on copy
    std::shared_ptr<int> val_;
    MoveExceptShared(int val) : val_{new int{val}} {}
    ~MoveExceptShared() {};
    MoveExceptShared(const MoveExceptShared& o) /*noexcept*/ : val_{new int{*o.val_.get()}} {};
    MoveExceptShared& operator=(const MoveExceptShared& o) /*noexcept*/ { val_.reset(new int{*o.val_.get()}); return *this; }
    MoveExceptShared(MoveExceptShared&&) = delete;
    MoveExceptShared& operator=(MoveExceptShared&&) = delete;
};
struct MoveNoExceptShared { // really copies all data on copy
    std::shared_ptr<int> val_;
    MoveNoExceptShared(int val) : val_{new int{val}} {}
    ~MoveNoExceptShared() {};
    MoveNoExceptShared(const MoveNoExceptShared& o) noexcept : val_{new int{*o.val_.get()}} {};
    MoveNoExceptShared& operator=(const MoveNoExceptShared& o) noexcept { val_.reset(new int{*o.val_.get()}); return *this; }
    MoveNoExceptShared(MoveNoExceptShared&&) = delete;
    MoveNoExceptShared& operator=(MoveExcept&&) = delete;
};

struct RuleOfZeroVector {
    std::vector<int> val_;
    RuleOfZeroVector(int val) : val_(val, val) {}
};
struct MoveExceptVector {
    std::vector<int> val_;
    MoveExceptVector(int val) : val_(val, val) {}
    MoveExceptVector(MoveExceptVector&& o) /*noexcept*/ : val_{} { swap(val_, o.val_); }
    MoveExceptVector& operator=(MoveExceptVector&& o) /*noexcept*/ { swap(val_, o.val_); return *this; }
};
struct MoveNoExceptVector {
    std::vector<int> val_;
    MoveNoExceptVector(int val) : val_(val, val) {}
    MoveNoExceptVector(MoveNoExceptVector&& o) noexcept : val_{} { swap(val_, o.val_); }
    MoveNoExceptVector& operator=(MoveNoExceptVector&& o) noexcept { swap(val_, o.val_); return *this; }
};



namespace {

constexpr size_t ITERATIONS = 1000*1000;

template<typename ELEM>
void timeStuff(std::string name) {
    cout << name << "...";
    auto start = steady_clock::now();
    std::vector<ELEM> data{}; // no reserve() on purpose
    for(size_t idx=0; idx<ITERATIONS; ++idx) {
        data.emplace_back( idx % 1719 );
    }
    cout << " " << millisSeit(start) << " ms";
    start = steady_clock::now();
    data.clear(); // remove all elements
    cout << " " << millisSeit(start) << " ms" << std::endl;
}

}

int main() {
    timeStuff<RuleOfZero>("RuleOfZero");
    timeStuff<DelMove>("DelMove");
    timeStuff<MoveExcept>("MoveExcept");
    timeStuff<MoveNoExcept>("MoveNoExcept");
    timeStuff<RuleOfZeroShared>("RuleOfZeroShared");
    timeStuff<MoveExceptShared>("MoveExceptShared");
    timeStuff<MoveNoExceptShared>("MoveNoExceptShared");
    timeStuff<RuleOfZeroVector>("RuleOfZeroVector");
    timeStuff<MoveExceptVector>("MoveExceptVector");
    timeStuff<MoveNoExceptVector>("MoveNoExceptVector");
}

/*
RuleOfZero... 192 ms 40 ms
DelMove... 196 ms 40 ms
MoveExcept... 188 ms 40 ms
MoveNoExcept... 187 ms 40 ms
RuleOfZeroShared... 145 ms 62 ms
MoveExceptShared... 247 ms 63 ms
MoveNoExceptShared... 232 ms 63 ms
RuleOfZeroVector... 2461 ms 242 ms
MoveExceptVector... 2472 ms 162 ms
MoveNoExceptVector... 2468 ms 162 ms

https://stackoverflow.com/questions/39710444/is-there-a-benefit-from-noexcept-move-operations-when-using-containers-in-c17
*/
