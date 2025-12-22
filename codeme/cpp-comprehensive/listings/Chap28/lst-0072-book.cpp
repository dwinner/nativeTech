
error_condition make_error_condition(errc c) noexcept {
  return error_condition(
      static_cast<int>(e),
      generic_category());
}
error_condition make_error_condition(io_errc c) noexcept /*...*/
error_condition make_error_condition(future_errc c) noexcept /*...*/