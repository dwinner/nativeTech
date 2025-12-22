
void process(const vector<string>& args) {
    if(args.size() == 0) {                 // expect parameters
        throw std::invalid_argument{"Command line argument missing"};
    } else {
        for(const string filename : args) {
            cout << filename << ": ";
            try {
                cout << countWords(filename) << "\n";
            } catch(std::exception &exc) {
                cout << "Error: " << exc.what() << "\n";
            }
        }
    }
}