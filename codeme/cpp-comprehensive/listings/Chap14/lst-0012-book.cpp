
// https://godbolt.org/z/j93cYaWd4 
#ifndef QWORT_H   // Header guard
#define QWORT_H
#include <string>
#include <string_view>
#include <memory> // unique_ptr
namespace qw {    // Namespace of the library

    int version();

    namespace impl_multimap {
        class index_impl;
    }

    class index {
        using index_impl = impl_multimap::index_impl;
    public:
        index();
        ~index() noexcept; // needed for pimpl
        index(const index&) = delete;
        index(index&&) noexcept;
        index& operator=(const index&) = delete;
        index& operator=(index&&) = delete;
    public:
        void add(std::string_view arg);
        size_t size() const;
        std::string getBestMatch(std::string_view query) const;
    public:          // public for tests
        std::string normalize(std::string arg) const;
    private:
        std::unique_ptr<index_impl> pimpl;
    };
} // namespace qw
#endif // Header guard