
// https://godbolt.org/z/P9z8n8v16 
#include <fstream> 
int main(int argc, char* argv[]) { 
    int value = 0; 
    std::ifstream myInput{"input1.txt"}; 
    myInput >> value; 
}