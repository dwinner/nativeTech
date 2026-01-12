
// return as (smart) pointer:
unique_ptr<Planet> createPlanet(const Event &evt) {
    unique_ptr<Planet> result{ new Planet{"Venus"} };  // heap object
    result->setLocation(evt.getPosition());
    return result; // passes the address
}