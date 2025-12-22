
// https://godbolt.org/z/Gxh4edc8n 
using std::vector;

struct B {};
struct D : public B {};

struct Base1 {
    virtual B& func();
};
struct Derived1 : public Base1 {
    virtual D& func() override;          // D& is covariant
};
struct Base2 {
    virtual B& func();
};
struct Derived2 : public Base2 {
    virtual D func() override;           // (ERR) D is not covariant
};
struct Base3 {
    virtual vector<B> func();
};
struct Derived3 : public Base3 {
    virtual vector<D>& func() override; // (ERR) vector<D>& is not covariant
};
struct Base4 {
    virtual vector<B*>& func();
};
struct Derived4 : public Base4 {
    virtual vector<D*>& func() override; // (ERR) different type, not covariant
};