
// https://godbolt.org/z/hEvqWqqc6 
int value = 5;                          // global variable
struct Wrap {
    int value = 3;                      // data field
    void set(int value) {               // parameter
        this->value = value + ::value;
    }
};