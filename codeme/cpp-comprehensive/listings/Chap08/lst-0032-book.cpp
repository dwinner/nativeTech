
// https://godbolt.org/z/Y4W11oW8b 
#include <string>
#include <vector>
#include <iostream>       // cout

void calculator(std::ostream& out, std::string input) {
    std::vector<int> stack {};
    for(char c : input) {
        if(c>='0' && c<='9') {
            stack.push_back( c-'0' ); // numeric value of the character
            continue;      // next loop iteration
        }
        int top = 0;
        int second = 0;
        switch(c)  {       // condition on character
        case '+':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second + top);
            break;
        case '-':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second - top);
            break;
        case '*':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second * top);
            break;
        case '=':
            for(int elem : stack) { out << elem; }
            out << " ";
            break;
        case ' ':
            break;
        default:
            out << " '" << c << "' I don't understand. ";
        } /* switch */
    } /* for c */
}
int main(int argc, const char* argv[]) {
    if(argc > 1) {
        calculator(std::cout, argv[1]);
    } else {
        // 3 + 4 * 5 + 6 with multiplication before addition results in 29
        calculator(std::cout, "345*+6+=");
    }
}