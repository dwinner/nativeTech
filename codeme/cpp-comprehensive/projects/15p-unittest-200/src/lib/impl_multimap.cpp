#include <map>
#include <string>

#include "impl_multimap.hpp" // self

#ifdef TDEBUG //# lines with '//#' are hidden in book print.
#include <iostream> //#
#define TRACE(x) std::clog << x; //#
#else //#
#define TRACE(x) //#
#endif //#


namespace qw {
namespace impl_multimap {

using std::vector;
using std::multimap;
using std::map;
using std::string;

using namespace std::literals::string_literals;


void index_impl::add(const string &normalized, const string& original) {
    // TODO: check for duplicate in 'entries'
    const auto pos = entries.size(); // index of new entry
    entries.push_back(original);
    auto qgrams = qgramify(normalized);
    for(const auto& qgram : qgrams) {
        qindex.insert( make_pair(qgram, pos) );
    }
}

string index_impl::getBestMatch(const string& normalized) const {
    auto qgrams = qgramify(normalized);
    // hits stores which indexed words have been partially matched how often
    map<size_t, size_t> hits; // 'entries-index' to 'hit-count'
    size_t maxhits = 0; // always: max(hits.second)
    for(const auto& qgram : qgrams) {
        auto range = qindex.equal_range(qgram);
        for(auto it=range.first; it!=range.second; ++it) {
            hits[it->second] += 1; // increment hit-count of entry
            if(hits[it->second] > maxhits) { // simplify search for max later
                maxhits = hits[it->second];
            }
        }
    }
    // find first entry with maxhits. Could be implemented using a prio-queue.
    for(auto const &hit : hits) {
        if(hit.second == maxhits) {
            return entries[hit.first];
        }
    }
    // will only be reached when entries is empty
    return ""s;
}

const string index_impl::PREFIX = string(Q-1, '^');
const string index_impl::SUFFIX = string(Q-1, '$');

vector<string> index_impl::qgramify(const string& normalized) const
{
    auto word = PREFIX + normalized + SUFFIX; // trick to get more/better qgrams
    TRACE("qwordify('"<<word<<"'):\n"); //#debug
    vector<string> result {};
    auto left = word.cbegin();
    auto right = std::next(word.cbegin(), Q); // valid, because |"^^"|+|"$$"| => 3.
    for( ; right <= word.end(); ++left, ++right) {
        TRACE(" '"<<(string(left,right))<<'\''); //#debug
        result.emplace_back(left, right);
    }
    TRACE('\n'); //#debug
    return result;
}

} // namespace impl_multimap
} // namespace qw

