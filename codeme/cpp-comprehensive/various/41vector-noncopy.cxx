#include <vector>

using std::vector;

struct DelCopy {
    int val_;
    DelCopy(int val) : val_{val} {}
    ~DelCopy() {};
    DelCopy(const DelCopy&) = delete;
    DelCopy& operator=(const DelCopy&) = delete;
    DelCopy(DelCopy&& o) noexcept : val_{o.val_} {};
    DelCopy& operator=(DelCopy&& o) noexcept { val_ = o.val_; return *this; }
};

struct NoCopy {
    int val_;
    NoCopy(int val) : val_{val} {}
    ~NoCopy() {};
    NoCopy(NoCopy&& o) noexcept : val_{o.val_} {};
    NoCopy& operator=(NoCopy&& o) noexcept { val_ = o.val_; return *this; }
};

struct None {
    int val_;
    None(int val) : val_{val} {}
    ~None() {};
};

struct DelMove {
    int val_;
    DelMove(int val) : val_{val} {}
    ~DelMove() {};
    DelMove(const DelMove& o) noexcept : val_{o.val_} {};
    DelMove& operator=(const DelMove& o) noexcept { val_ = o.val_; return *this; }
    DelMove(DelMove&&) = delete;
    DelMove& operator=(DelMove&&) = delete;
};

struct NoMove {
    int val_;
    NoMove(int val) : val_{val} {}
    ~NoMove() {};
    NoMove(const NoMove& o) noexcept : val_{o.val_} {};
    NoMove& operator=(const NoMove& o) noexcept { val_ = o.val_; return *this; }
};

struct DelAll {
    int val_;
    DelAll(int val) : val_{val} {}
    ~DelAll() {};
    DelAll(const DelAll&) = delete;
    DelAll& operator=(const DelAll&) = delete;
    DelAll(DelAll&&) = delete;
    DelAll& operator=(DelAll&&) = delete;
};

struct NoDestroy {
    int val_;
    NoDestroy(int val) : val_{val} {}
    ~NoDestroy() = delete;
};


//no:
// static vector<NoDestroy> kept{ NoDestroy{3} };
// static vector<NoDestroy> keptx{ };

int main() {
    vector<DelCopy> delcopies{};
    delcopies.emplace_back( 4 );
    delcopies.push_back( 5 );

    vector<DelMove> delmoves{};
    delmoves.emplace_back( 6 );
    //no:
    // delmoves.push_back( 7 );

    vector<NoMove> nomoves{};
    nomoves.emplace_back( 8 );
    nomoves.push_back( 9 );

    vector<NoCopy> nocopies{};
    nocopies.emplace_back( 10 );
    nocopies.push_back( 11 );

    vector<None> nones{};
    nones.emplace_back( 13 );
    nones.push_back( 14 );

    vector<DelAll> delalls{  };
    //no:
    // delalls.emplace_back( 16 );
    //no:
    // delalls.push_back( 17 );

    //timeStuff();
}
