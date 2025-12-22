
if(number > 50) {
    int result = number*number;        // Definition of result
    std::cout << "Square: " << result << std::endl;
    int result = number+number;        // (ERR) Error: result has already been defined
    std::cout << "Doubled: " << result << std::endl;
}