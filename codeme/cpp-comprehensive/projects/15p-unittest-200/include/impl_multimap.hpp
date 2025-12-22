#include <vector>
#include <map> // multimap

namespace qw {
namespace impl_multimap {

using std::vector;
using std::multimap;
using std::string;

class index_impl {
    vector<string> entries;
    multimap<string,size_t> qindex;
public:
    void add(const string &normalized, const string& original);
    string getBestMatch(const string& normalized) const;
    size_t size() const {
        return entries.size();
    }
private:
    vector<string> qgramify(const string& normalized) const;
    static constexpr size_t Q = 3;
    static const std::string PREFIX;
    static const std::string SUFFIX;
public: // test interface
    vector<string> _qgramify(const string& n) const { return qgramify(n); }
    static size_t _q() { return Q; }
    static std::string _prefix() { return PREFIX; }
    static std::string _suffix() { return SUFFIX; }
};

} // namespace impl_multimap
} // namespace qw

