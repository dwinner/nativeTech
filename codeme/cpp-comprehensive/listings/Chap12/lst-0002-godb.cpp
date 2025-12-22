//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/zPbK16zWE 
#include <memory>                   // shared_ptr
#include <vector>                   // vector
struct Employee;                    // class declaration
struct Boss;                        // class declaration
struct Employee {                   // class definition
    std::shared_ptr<Boss> boss_;    // pointer to Boss
    void print() const;             // method declaration
};
struct Boss {                       // definition
    std::vector<std::shared_ptr<Employee>> employees_; // Pointer to Employees
    void print() const;             // method declaration
};
void Employee::print() const {      // method definition
    // …
}
void Boss::print() const {          // method definition
    // …
}