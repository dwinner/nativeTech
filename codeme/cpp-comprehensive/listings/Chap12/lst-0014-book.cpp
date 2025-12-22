
string greeting(Person * const p) {  // implicit parameter made explicit
    return format("Hello {} from {}", p->name_, p->city_);
}