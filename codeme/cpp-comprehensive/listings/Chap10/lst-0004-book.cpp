
try {
    cout << countWords(filename) << "\n";
} catch(std::exception &exc) {
    cout << "Error: " << exc.what() << "\n";
    throw; // rethrow
}