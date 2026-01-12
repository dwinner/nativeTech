//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6cqd7Tsae 
struct BackgroundTask {
    void operator()(std::stop_token st) const { // Token for communication
        task1();
        if(st.stop_requested()) return;
        task2();
        if(st.stop_requested()) return;
        task3();
    }
};
int main() {
    BackgroundTask backgroundTask{};
    std::jthread myThread{ backgroundTask };
    std::this_thread::sleep_for(std::chrono::milliseconds(100)); // wait 100ms
    myThread.request_stop(); // request the thread to stop
}