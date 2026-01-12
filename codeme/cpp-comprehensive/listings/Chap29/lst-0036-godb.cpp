//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TcsrefEPh 
#include <mutex> // once_flag, call_once
#include <memory>
struct Connection {
    void csend(const char *data) {} // dummy
    const char* crecv() {} // dummy
};
class Sender {
    std::shared_ptr<Connection> conn_;
    std::once_flag connInitFlag_;
    void open() {
      conn_.reset( new Connection{} );
    }
public:
    void send(const char* data) {
      std::call_once(connInitFlag_, &Sender::open, this); // method pointer
      conn_->csend(data);
    }
    const char* recv() {
      std::call_once(connInitFlag_, [this] {this->open();} ); // lambda
      return conn_->crecv();
    }
};