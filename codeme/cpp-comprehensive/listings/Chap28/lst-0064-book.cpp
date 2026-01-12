
// https://godbolt.org/z/rz38hc1cx 
auto res = fib(45);
// …
std::cout << "Time: " << duration_cast<seconds>(t1-t0).count() << "s\n";
// Output: Time: 7s (e.g.)
std::cout << "Time: " << duration<double>{t1-t0}.count() << "s\n";
// Output: Time: 7.35303s (e.g.)
}