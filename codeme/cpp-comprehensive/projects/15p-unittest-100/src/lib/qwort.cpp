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

    namespace impl_simple_map {

            void index_impl::add(const string &normalized, const string& original) {
                data[normalized] = original;
            }

            size_t index_impl::size() const {
                return data.size();
            }

            string index_impl::getBestMatch(const string& normalized) const {
                const auto where = data.find(normalized);
                return where != data.end() ? where->second : "";
            }

    } // namespace impl_map


    // === Index main class ===

    index::index()
        : pimpl{ new index_impl{} }
        { }

    // A non-default destructor in the header allows the use of an
    //   incomplete type with unique_ptr, and thus the nice use
    //   of the Pimpl pattern. http://stackoverflow.com/a/9064437/472245
    index::~index() {
    }

    void index::add(const string &arg) {
        pimpl->add(normalize(arg), arg);
    }

    string index::normalize(string str) const {
        using namespace std; // begin, end
        transform(begin(str), end(str), begin(str), [](char c) {
                return ::isalpha(c) ? ::toupper(c) : '#';
            });
        return str;
    }

    size_t index::size() const {
        return pimpl->size();
    }

    string index::getBestMatch(const string& query) const {
        return pimpl->getBestMatch(normalize(query));
    }


} // namespace qw
