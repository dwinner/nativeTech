#ifndef ENCODING_MECHANISM01_H
#define ENCODING_MECHANISM01_H

class Widget
{
};

inline Widget widgetFactory() { return {}; } // function returning rvalue

template<typename T>
void func(T &&param)
{
}

#endif /* ENCODING_MECHANISM01_H */
