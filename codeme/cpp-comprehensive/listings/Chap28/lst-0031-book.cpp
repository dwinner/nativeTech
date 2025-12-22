
// https://godbolt.org/z/173nr4xxd 
sleep(10min);
sleep(std::chrono::minutes{10});
sleep(std::chrono::duration<unsigned long,std::ratio<60>>{10});