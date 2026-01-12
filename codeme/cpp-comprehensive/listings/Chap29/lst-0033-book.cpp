
// https://godbolt.org/z/e7rraPGx4 
#include <thread>
#include <mutex>
#include <vector>
#include <numeric> // accumulate, iota
using std::mutex; using std::unique_lock;

std::vector<int> myData;              // shared data
mutex myMutex;                        // Mutex for the data
unique_lock<mutex> prepareData() {
    unique_lock lk1{myMutex};        // lock
    myData.resize(1000);
    std::iota(myData.begin(), myData.end(), 1); // 1..1000
    return lk1;                                 // Transfer lock
}
int processData() {
    unique_lock lk2 = prepareData();         // Lock transferred
    return std::accumulate(myData.begin(), myData.end(), 0);
}