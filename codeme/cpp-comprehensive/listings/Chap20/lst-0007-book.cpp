
Planet createPlanet(const Event &evt) {  // return by value
    Planet result{"Earth"};              // stack object
    result.setLocation(evt.getPosition());
    return result;                       // return creates (potential) copy
}