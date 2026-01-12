
// https://godbolt.org/z/Tzs93MzfK 
std::thread myThread{ [] {  // pure thread
    task1();
    task2();
    task3();
} };
myThread.join();   // waits for the thread to finish