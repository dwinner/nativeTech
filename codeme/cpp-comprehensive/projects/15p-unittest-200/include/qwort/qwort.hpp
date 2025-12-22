#ifndef QWORT_H
#define QWORT_H
#include <string>
#include <memory> // unique_ptr

#include "impl_multimap.hpp" // real this time

namespace qw {

    int version();

    class index {
        using index_impl = impl_multimap::index_impl; //+ real this time
    public:
        index();
        ~index() noexcept; // http://stackoverflow.com/a/9064437/472245
        index(const index&) = default;
        index(index&&) noexcept;
        index& operator=(const index&) = default;
        index& operator=(index&&) = default;
    public:
        void add(const std::string &arg);
        size_t size() const;
        std::string getBestMatch(const std::string& query) const;
    public: // public for tests, normally would be private or protected
        std::string normalize(std::string arg) const;
    private:
        std::shared_ptr<index_impl> pimpl;
    };

} // namespace qw
#endif
