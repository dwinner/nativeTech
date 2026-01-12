
// https://godbolt.org/z/67cd5bhhE 
#include <string>
#include <iostream> // cout
using std::string; using std::to_string; using std::cout;
void triggerError(int errorCase) {
   try {
       if(errorCase < 10) throw (int)errorCase;
       else if(errorCase < 20) throw 1.0/(errorCase-10.0);
       else throw string{"Error " + to_string(errorCase)};
   } catch(int eval) {
       cout << "int-error: " << eval << "\n";
   } catch(double eval) {
       cout << "double-error: " << eval << "\n";
   } catch(string eval) {
       cout << "string-error: " << eval << "\n";
   }
}
int main() {
    triggerError(3);   // Output: int-error: 3
    triggerError(14);  // Output: double-error: 0.25
    triggerError(50);  // Output: string-error: Error 50
}