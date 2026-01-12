
namespace plant {
    // … as before …
    std::ostream& operator<<(std::ostream&, const Tree&) {…};
    namespace debug {
        std::ostream& operator<<(std::ostream&, const Tree&) {…};
    }
}
plant::Tree tree{"MyTree"};
void run() {
    using namespace plant;
    cout << tree << "\n";
}
void diagnostic() {
    using namespace plant::debug;
    cout << tree << "\n";
}
int main() {
    run();
    diagnostic();
}