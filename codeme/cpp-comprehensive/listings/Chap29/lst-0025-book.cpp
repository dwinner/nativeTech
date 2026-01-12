
// https://godbolt.org/z/3fz5a4vYv 
#include <mutex> // mutex, lock_guard
#include <list>
#include <algorithm> // find
using std::lock_guard; using std::mutex; using std::find;
class MxIntList {
    std::list <int> data_;
    mutable mutex mx_;
public:
    void add(int value) {
        lock_guard guard{mx_};  // protects until the end of the method
        data_.push_back(value);
    }
    bool contains(int searchVal) const {
        lock_guard guard{mx_};  // protects until the end of the method
        return find(data_.begin(), data_.end(), searchVal) != data_.end();
    }
};