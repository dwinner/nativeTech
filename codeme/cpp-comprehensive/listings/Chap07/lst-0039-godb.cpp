//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8Gdso36zf 
class Widget : public Base {
    explicit Widget(int);      // no automatic conversion from int
    ~Widget();                 // Destructor with ~ before the name
    virtual void update();     // prefixed with virtual
    void calc1() override;     // suffixed with override
    void calc2() final;        // suffixed with final
    void draw() const;         // suffixed with const
    virtual void paint() = 0;  // abstract method
};