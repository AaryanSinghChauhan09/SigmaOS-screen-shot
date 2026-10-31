# SigmaOS Screenshot

This crate models screenshot configuration and generates a numbered output
path. `capture()` increments an in-memory counter and returns that path; it does
not capture pixels, contact a display server, write an image, or copy to the
clipboard. The format and capture-mode enums currently describe requested
options only.

The unit tests cover filename extensions and counter behavior, not screenshot
capture or image encoding.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
