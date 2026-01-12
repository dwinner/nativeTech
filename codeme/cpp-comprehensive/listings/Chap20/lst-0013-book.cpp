
// https://godbolt.org/z/4KP91z1bq
#include <vector>
#include <iostream>
#include <memory>                    // shared_ptr
#include <random>                    // uniform_int_distribution, random_device
namespace {                          // Beginning of the anonymous namespace
using std::shared_ptr; using std::make_shared;
using std::vector; using std::cout;
struct Asteroid {
    int points_ = 100;
    int structure_ = 10;
};
struct Ship {
    shared_ptr<Asteroid> firedLastOn_{};
    int score_ = 0;
    int firepower = 1;
    bool fireUpon(shared_ptr<Asteroid> a);
};
struct GameBoard {
    vector<shared_ptr<Asteroid>> asteroids_;
    explicit GameBoard(int nAsteroids);
    bool shipFires(Ship& ship);
};
// Implementation of Ship
bool Ship::fireUpon(shared_ptr<Asteroid> a) {
    if(!a) return false;             // invalid asteroid
    a->structure_ -= firepower;
    if(a.get() == firedLastOn_.get())
        firepower *= 2 ;             // increase damage
    else
        firepower = 1;               // reset
    firedLastOn_ = a;
    return a->structure_ <= 0;       // destroyed?
}
// Implementation of GameBoard
GameBoard::GameBoard(int nAsteroids) : asteroids_{}
{   // some standard asteroids
    for(int idx=0; idx<nAsteroids; ++idx)
        asteroids_.push_back( make_shared<Asteroid>() );
}
int rollDice(int min, int max) {
    /* static std::default_random_engine e{}; */  // Pseudo-random generator
    static std::random_device e{};   // random generator
    return std::uniform_int_distribution<int>{min, max}(e); // roll dice
}
bool GameBoard::shipFires(Ship &ship) {
    int idx = rollDice(0, asteroids_.size()-1);
    bool broken = ship.fireUpon(asteroids_[idx]);
    if(broken) {
        ship.score_ += asteroids_[idx]->points_;
        asteroids_.erase(asteroids_.begin()+idx);  // remove
    }
    return asteroids_.size() == 0;                 // everything is destroyed
}
} // End of the anonymous namespace
int main() {
    GameBoard game{10};                            // 10 asteroids
    Ship ship{};
    for(int idx = 0; idx < 85; ++idx) {            // 85 shots
        if(game.shipFires(ship)) {
            cout << "The space is empty after " << idx+1 << " shots. ";
            break;
        }
    }
    cout << "You have scored " << ship.score_ << " points.\n";
}