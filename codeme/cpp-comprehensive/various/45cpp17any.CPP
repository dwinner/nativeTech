    #include <any>
    #include <iostream>
    #include <vector>
    #include <string>
    int main() {
        std::any a = 5;
        std::cout << std::any_cast<int>(a) << '\n';
        //= 5
        a = 3.456;
        std::cout << std::any_cast<double>(a) << '\n';
        using namespace std::literals;
        std::vector<std::any> data { 4, 8.976, "Geronimo"s };
        std::cout << std::any_cast<double>( data[1] ) << '\n';
        std::cout << data[1].type().name()  << '\n';
    }
