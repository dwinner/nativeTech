
// https://godbolt.org/z/K9c913had 
if(int result; (result = read(buffer, 100)) != 0) {
    std::cerr << "Error number "<< result << " occurred.\n";
}