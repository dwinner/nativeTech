
if(number > 50) {                // outer block
    {                            // 1st inner block
        int result  = number*number;
        std::cout << "Square: " << result << std::endl;
    }
    {                            // 2nd inner block
        int result  = number+number;
        std::cout << "Doubled: " << result << std::endl;
    }
}