//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xWr7axonx
#include <iostream>           // cout
#include <memory>             // unique_ptr
using std::cout;
class Driver {                // abstract base class
public:
    virtual void init() = 0;
    virtual void done() = 0;
    virtual void send(const std::string &data) = 0;
};
class ProductionDriver : public Driver {
public:
    void init() override { }
    void done() override { }
    void send(const std::string &data) override { cout << data << "\n"; }
};
class DebuggingDriver : public Driver {
   size_t countSend_ = 0;
public:
    void init() override {
        countSend_= 0; cout << "Ok, I'm initialized.\n";
    }
    void done() override {
        cout << "send used:" << countSend_ << " times\n";
    }
    void send(const std::string &data) override {
        cout << "send("<<countSend_<<"):"<< data << "\n";
        ++countSend_;
    }
};
struct DriverWrapper {        // RAII wrapper for init() and done()
    Driver &driver_;
    explicit DriverWrapper(Driver& driver) : driver_(driver) {
        driver_.init(); }
    ~DriverWrapper() { driver_.done(); }
    DriverWrapper(const DriverWrapper&) = delete; // no copying
};

void doWork(Driver &driver) { // someone who flexibly uses any driver
    DriverWrapper wrapper(driver);      // call init() and done()
    driver.send("An Unexpected Journey");
    driver.send("The Desolation Of Smaug");
}

int main() {
    // same doWork, once with production and once with debugging driver
    ProductionDriver production{};
    doWork( production );
    DebuggingDriver debugging{};
    doWork( debugging );

    // more common variant of a dynamically created driver
    std::unique_ptr<Driver> driver{ new ProductionDriver{} };
    doWork( *driver );
}