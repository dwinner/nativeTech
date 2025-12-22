#include "regexpr.hpp"

#include <iostream>

#include <cstring> // strlen

using std::cerr; using std::endl;

xsyntax _xsyntax;

// RegExpr::Deque
RegExpr::Deque::Deque()
{
    head = 0;
    tail = 0;
}

RegExpr::Deque::~Deque()
{
    while(head) {
        tail = head;
        head = head->next;
        delete tail;
    }
}

void RegExpr::Deque::push(int n)
{
    Element *ptr = new Element;
    ptr->n = n;
    ptr->next = head;
    if(!tail)
        tail = ptr;
    head = ptr;
}

void RegExpr::Deque::put(int n)
{
    Element *ptr = new Element;
    ptr->n = n;
    ptr->next = 0;
    if(!head)
        head = ptr;
    else
        tail->next = ptr;
    tail = ptr;
}

int RegExpr::Deque::pop()
{
    int ret = head->n;
    Element *ptr = head;
    head = head->next;
    if(!head)
        tail = 0;
    delete ptr;
    return ret;
}

inline int RegExpr::Deque::isEmpty()
{
    return head ? 0 : 1;
}

// RegExpr
RegExpr::RegExpr()
{
    automaton[0].next1 = 0;
    automaton[0].next2 = 0;
}

RegExpr::RegExpr(const char *pattern)
{
    compile(pattern);
}

void RegExpr::clear_automaton()
{
    for(int n=0;n<MAXSTATES;n++) {
        automaton[n].the_char = '\0';
        automaton[n].next1 = 0;
        automaton[n].next2 = 0;
    }
}

// error handling
void RegExpr::clear_after_error()
{
    switch(_error_type) {
    case error::compiler_error:
        clear_automaton();
        break;
    case error::simulation_error:
        while(!deque.isEmpty())
            deque.pop();
        break;
    case error::no_error:
        break;
    }
    _error_type = error::no_error;
}

// compiler
inline int RegExpr::isLetter(char c)
{
    return (c != '|' && c != '(' && c != ')' && c != '*' && c != '\\' && c != '\0' ? 1 : 0);
}

void RegExpr::compile(const char *pattern)
{
    set_error_type(error::compiler_error);

    p = pattern;

    state = 0;
    j = 0;

    clear_automaton();
    int n = list();
    if(automaton[0].next1 == 0)
        automaton[0].next1 = automaton[0].next2 = n;
    automaton[state].next1 = automaton[state].next2 = 0;

    for(auto a : automaton) {
        if(a.the_char==0 && a.next1==0 && a.next2==0)
            break;
    }

    set_error_type(error::no_error);
}

unsigned RegExpr::list()
{
    unsigned s1 = state++, s2 = element(), s3;
    if(p[j] == '|'){
        j++;
        s3 = ++state;
        automaton[s3].next1 = s2;
        automaton[s3].next2 = list();
        automaton[s3-1].next1 = automaton[s3-1].next2 = state;
        if(automaton[s1].next1 == s2 || automaton[s1].next1 == 0)
            automaton[s1].next1 = s3;
        if(automaton[s1].next2 == s2 || automaton[s1].next2 == 0)
            automaton[s1].next2 = s3;
        return s3;
    }
    return s2;
}

unsigned RegExpr::element()
{
    unsigned s1 = state, s2;

    if(p[j] == '('){
        j++;
        s2 = list();
        if(p[j] == ')') {
            automaton[state].next1 = automaton[state].next2 = state+1;
            state++;
            j++;
        } else
            error();
    } else
        s2 = v();

    if(p[j] == '*') {
        automaton[state].next1 = s2;
        automaton[state].next2 = state+1;
        s1 = state;
        if(automaton[s2-1].next1 == s2 || automaton[s2-1].next1 == 0)
            automaton[s2-1].next1 = state;
        if(automaton[s2-1].next2 == s2 || automaton[s2-1].next2 == 0)
            automaton[s2-1].next2 = state;
        state++;
        j++;
    } else {
        if(automaton[s1-1].next1 == 0)
            automaton[s1-1].next1 = s2;
        if(automaton[s1-1].next2 == 0)
            automaton[s1-1].next2 = s2;
    }

    if(p[j] != '|' && p[j] != ')' && p[j] != '*' && p[j] != '\0')
        element();

    return s1;
}

unsigned RegExpr::v()
{
    unsigned s1 = state;
    char *s;

    if(p[j] == '\\')
        j++;
    else if(!isLetter(p[j]))
        error();

    automaton[state].the_char = p[j++];
    automaton[state].next1 = automaton[state].next2 = state+1;
    state++;
    return s1;
}

void RegExpr::error()
{
    _xsyntax.setErrorPos(j);
    throw _xsyntax;
}

// automaton simulation
int RegExpr::searchLen(const char *str, unsigned &len, unsigned start)
{
    if(automaton[0].next1 == 0 && automaton[0].next2 == 0)
        return REGEXPR_NOT_COMPILED;

    set_error_type(error::simulation_error);

    int n, m, slen = strlen(str);
    for(n = start;n < slen;n++){
        m = simulate(str, n);
        if(m < n - 1) {
            len = m - n + 1;
            set_error_type(error::no_error);
            return n;
        }
    }

    set_error_type(error::no_error);

    return REGEXPR_NOT_FOUND;
}

#define next_char       -1
int RegExpr::simulate(const char *str, int j)
{
    int state = automaton[0].next1,
        last_match = j - 1,
        len = strlen(str);

    if(automaton[0].next1 != automaton[0].next2)
        deque.push(automaton[0].next2);
    deque.put(next_char);
    do {
        if(state == next_char) {
            if(str[j])
                j++;
            deque.put(next_char);
        } else if(automaton[state].the_char == str[j]) {
            deque.put(automaton[state].next1);
            if(automaton[state].next1 != automaton[state].next2)
                deque.put(automaton[state].next2);
        } else if(!automaton[state].the_char) {
            deque.push(automaton[state].next1);
            if(automaton[state].next1 != automaton[state].next2)
                deque.push(automaton[state].next2);
        }
        state = deque.pop();
        if(state == 0) {
            last_match = j - 1;
            state = deque.pop();
        }
    } while(j <= len && !deque.isEmpty());

    return last_match;
}
#undef next_char
