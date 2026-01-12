
class Mega {
    std::vector<int>     data_;
    CanThrow             canThrow_;
    std::map<string,int> more_;
public:
    Mega()
      : data_{}
      , canThrow_{666}  // triggers an exception
      , more_{}
      { }
};