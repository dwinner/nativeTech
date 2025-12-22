
// https://godbolt.org/z/3vcnzfqfM 
#include <mutex> // once_flag, call_once
std::shared_ptr<BigData> bigData{};
std::once_flag bigDataInitFlag;
void initBigData() {
    bigData = std::make_shared<BigData>();
}
int useBigData() {
    std::call_once(bigDataInitFlag, initBigData);
    // bigData->…
}