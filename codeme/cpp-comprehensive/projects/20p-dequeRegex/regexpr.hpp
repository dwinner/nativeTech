#ifndef __REGEXPR_H_OGM
#define __REGEXPR_H_OGM

class xsyntax
{
public:
    unsigned getErrorPos(){ return errorPos; }
    void setErrorPos(unsigned pos){ errorPos = pos; }
protected:
    unsigned errorPos;
};

#define MAXSTATES       100

class RegExpr
{
public:
    RegExpr();
    explicit RegExpr(const char *pattern);

    void compile(const char *pattern);
    int search(const char *str, unsigned start = 0);
    int searchLen(const char *str, unsigned &len, unsigned start = 0);

    const char *operator = (const char *pattern);

    void clear_after_error();

private:
    enum class error {
        no_error = 0,
        compiler_error = 1,
        simulation_error = 2,
    };

    error _error_type;

    void set_error_type(error et) { _error_type = et; }

protected:
    struct State
    {
        char the_char;
        unsigned next1, next2;
    } automaton[MAXSTATES];

    void clear_automaton();

    class Deque
    {
    public:
        Deque();
        ~Deque();

        void push(int n);
        void put(int n);
        int pop();

        int isEmpty();

    protected:
        struct Element {
            int n;
            Element *next;
        } *head, *tail;
    } deque;

    // compiler
    unsigned j, state;
    const char *p;

    unsigned list();
    unsigned element();
    unsigned v();
    void error();

    int isLetter(char c);

    // automaton simulation
    int simulate(const char *str, int j);
};


inline const char *RegExpr::operator = (const char *pattern)
{
    compile(pattern);
    return pattern;
}

inline int RegExpr::search(const char *str, unsigned start)
{
    unsigned dummy = 0;
    return searchLen(str, dummy, start);
}

// error codes of RegExpr::search and RegExpr::searchLen
#define REGEXPR_NOT_FOUND               -1
#define REGEXPR_NOT_COMPILED    -2

#endif
