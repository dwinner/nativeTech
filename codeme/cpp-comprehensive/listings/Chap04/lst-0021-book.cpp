
if(argc<=1) {                                // start of the if statement 
    std::cout << "Enter a number: "; 
    std::cin >> number; 
    if(!std::cin) { 
        return 1; 
    } 
} else { 
    value = std::stoi(argv[1]); 
}                                            // end of the if statement