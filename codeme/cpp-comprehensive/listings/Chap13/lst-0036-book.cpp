
// https://godbolt.org/z/4dnTs4vPx
namespace {
    const int MAX_A = 12;       // the same as MAX_B, but no static needed
}
static const int MAX_B = 10;    // in the global namespace
struct Data {
    static const int SIZE = 14; // as a data field in a class
};

void func() {
    static const int LIMIT = 16; // as a local constant
}