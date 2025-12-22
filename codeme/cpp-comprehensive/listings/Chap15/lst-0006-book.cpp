
// https://godbolt.org/z/eYY1js4Te 
class Year : public Value {               // derive from class Value
public:
    explicit Year(int v) : Value{v, 4} {} // initialize base class
};
class Month : public Value {
public:
    explicit Month(int v) : Value{v, 2} {}
};
struct Day : public Value {               // class-public corresponds to struct
    explicit Day(int v) : Value{v, 2} {}
};