
template<auto ... vs> struct MixedList {};
using Three = MixedList<'a', 100, 'b'>;
Three three{};