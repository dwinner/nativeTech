
// https://godbolt.org/z/6vsqnWeWh 
struct Base {
    virtual ~Base() {};  // define the destructor, make it virtual
    virtual void other()
};

struct Derived : public Base {
    void other() override
};

int main() {
    Base *obj = new Derived{};
    /* ... more lines of code here ... */
    delete obj;          // works because Base::~Base is virtual
}