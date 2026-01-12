//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/s65GvY9xj
struct MyValue {
    // … everything else as before
    MyValue& operator=(const MyValue& right) {
        if(this != &right) { // 1. check for self-assignment
            // 2. Release previous resources; none here
            // 3. element-wise transfer by assignment or similar
            name_ = right.name_ + "-Assignment (previously " + name_ + ")";
            /* number_ remains, and thus the original indentation */
        }
        return *this; // 4. return self
    }
};