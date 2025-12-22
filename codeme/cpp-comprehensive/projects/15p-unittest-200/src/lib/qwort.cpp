#include "qwort/qwort.hpp" // self

#include <map>
#include <algorithm> // transform
#include <cctype> // toupper

using std::map;
using std::string;

namespace qw {

    int version() {
        return 1;
    }

    // === Index main class ===

    index::index()
        : pimpl{ new index_impl{} }
        { }

    // A non-default destructor in the header allows the use of an
    //   incomplete type with unique_ptr, and thus the nice use
    //   of the Pimpl pattern. http://stackoverflow.com/a/9064437/472245
    //   Unfortunately, the compiler-generated default implementation
    //   does not work. But if we do this in the cpp file _after_ including
    //   the implementation, then it works.
    index::~index() noexcept = default;
    index::index(index&&) noexcept =default;

    void index::add(const string &arg) {
        pimpl->add(normalize(arg), arg);
    }

    size_t index::size() const {
        return pimpl->size();
    }

    string index::getBestMatch(const string& query) const {
        return pimpl->getBestMatch(normalize(query));
    }

    string index::normalize(string str) const {
        using namespace std; // begin, end
        transform(begin(str), end(str), begin(str), [](char c) {
                return ::isalpha(c) ? ::toupper(c) : '#';
            });
        return str;
    }

} // namespace qw