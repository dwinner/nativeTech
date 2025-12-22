//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EnE7Y886M 
#include <iostream>
#include <vector>
using std::cout;
// == Observer Design Pattern ==
struct Observer {
    virtual void update() = 0;
};
class Subject {
    std::vector<Observer*> observers_; // not owning pointers
protected:
    void notify() {
        for (auto o : observers_)
            o->update();
    }
public:
    void addObserver(Observer* o) {
        observers_.push_back(o);
    }
};
// == Concrete Class ==
struct MyThing {
    int calc() { return 1+1; }
};
// == bring together ==
struct MyObservableThing : public MyThing, public Subject {
    int calc() {
        notify();
        return MyThing::calc();
    }
};
// == observe something ==
struct MyObserver : public Observer {
    void update() override {
        cout << "observed\n";
    }
};
int main() {
    MyObserver myObserver{};
    MyObservableThing myObservableThing{};
    myObservableThing.addObserver(&myObserver);
    auto result = myObservableThing.calc(); // Output: observed
}