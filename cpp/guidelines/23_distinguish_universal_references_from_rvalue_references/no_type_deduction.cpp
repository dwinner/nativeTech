/*
 * Key idea:
 *
 *   If you see "T&&" without type deduction, you're looking at a rvalue
 *   reference.
 */

class Widget
{
};


void f(Widget &&param); // no type deduction;
// param is a rvalue reference

Widget &&var1 = Widget(); // no type deduction;
// var1 is a rvalue reference
