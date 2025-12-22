
// https://godbolt.org/z/rW4oTPhrG 
struct Driver {
    virtual void init() = 0;
    virtual void done() = 0;
    virtual bool send(const char* data, unsigned len) = 0;
};