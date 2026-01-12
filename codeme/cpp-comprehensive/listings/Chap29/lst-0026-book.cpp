
// https://godbolt.org/z/4j7njd763 
#include <vector>

template<class T>
class MxStack {
public:
    bool isEmpty() const;
    void push(const T&);
    void pop();
    const T& top() const;
};