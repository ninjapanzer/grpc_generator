// Inside src/package/mod.rs

// Declare the python module, which will look for a file named `python.rs` in the same directory.
pub mod python;
pub mod ruby;
pub mod oas;
mod python_parser;