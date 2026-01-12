
if constexpr std::integral<T> {
  // T is an integral type
} else if constexpr std::floating_point<T> {
  // T is a floating-point type
} else {
  // T is neither
}