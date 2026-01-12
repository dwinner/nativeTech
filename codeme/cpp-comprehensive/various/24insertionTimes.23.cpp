#include <iostream>
#include <vector>
#include <chrono>

int main() {
    // Number of insertions to test
    std::vector<int> insertionCounts = {10, 100, 1000, 10'000, 100'000, 1'000'000, 10'000'000, 100'000'000};

    std::cout << "# test-case";
    for (int numInsertions : insertionCounts) {
        std::cout << "," << numInsertions;
    }
    std::cout << "\n";

    for(int i = 1; i <= 6; ++i) {
        if(i % 2 == 0) {
            std::cout << "reserve";
        } else {
            std::cout << "plain";
        }

        for (int numInsertions : insertionCounts) {
            auto start = std::chrono::high_resolution_clock::now();
            std::vector<int> data;
            if(i % 2 == 0) {
                data.reserve(numInsertions);
            }
            for (int i = 0; i < numInsertions; ++i) {
                data.push_back(i);
            }
            auto end = std::chrono::high_resolution_clock::now();
            std::chrono::duration<double, std::milli> duration = end - start;

            std::cout << "," << duration.count();;
        }
        std::cout << "\n";
    }
}
