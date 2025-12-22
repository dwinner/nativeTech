
// https://godbolt.org/z/f8on7bs1Y
string drivingSchool(Stoplight light) {
    switch(light.color) {
      using enum StoplightColor;   // "StoplightColor::" can now be omitted
      case RED: return "stop";
      case REDYELLOW: return "get ready";
      case YELLOW: return "slow down";
      case GREEN: return "go";
    }
    abort(); // exhaustive switch, should not happen
}