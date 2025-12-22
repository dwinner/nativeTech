
// https://godbolt.org/z/9PMcz1cqP 
friend void swap(MxStack& re, MxStack& li) {
    if(&re==&li) return; // Same address? Swapping with itself is unnecessary
    std::lock( re.mx_, li.mx_ );   // multiple locks simultaneously
    std::lock_guard lkre{re.mx_, std::adopt_lock}; // already locked
    std::lock_guard lkli{li.mx_, std::adopt_lock}; // already locked
    std::swap(li.data_, re.data_); // perform swap
}