//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/e5GWbbq4Y
#ifndef QWORT_H // header guard
#define QWORT_H
#include <string>
#include <memory> // unique_ptr
namespace qw { // library namespace
    int version();
    namespace impl_multimap {
        class index_impl;
    }
    class index {
        using index_impl = impl_multimap::index_impl;
    public:
        index();
        ~index() noexcept; // needed for pimpl
        index(const index&) = default;
        index(index&&) noexcept = default;
        index& operator=(const index&) = default;
        index& operator=(index&&) = default;
    public:
        void add(const std::string &arg);
        size_t size() const;
        std::string getBestMatch(const std::string& query) const;
    public:          // public for tests
        std::string normalize(std::string arg) const;
    private:
        const std::unique_ptr<index_impl> pimpl;
    };
} // namespace qw
#endif // header guard