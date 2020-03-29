# rustyPixelGameEngine
A port of olcPixelGameEngine to Rust, a tool used in javidx9's YouTube videos and projects. This port keeps the original license and the link to documentation is straight to the official project.

![Rust](https://github.com/ElementG9/rustyPixelGameEngine/workflows/Rust/badge.svg)

# Why port to Rust?
  * Why not
  * Rewriting the algorithms makes them easier to understand
  * Rust's tooling is much better than C++'s

# Goals
* Obtain close to feature parity with PGE
* Beat PGE's speed (cheating allowed 😉)
* Try to stay close to the API style of PGE, but I will stray if it makes sense

# Differences
* Many things may not be implemented
* Uses a Rust crate called [minifb](https://github.com/emoon/rust_minifb) to handle the window creation and event code instead of SDL
* Debug mode is painfully slow, this is mostly a Rust problem
* Includes some extra folders that can be ignored
  * .cargo - Contains a cargo config file used to test for speed
  * .vscode - A couple json files used with VSCode

# Usage
* Install latest stable [Rust](https://www.rust-lang.org/)
* Run `cargo run --example extensiontestgfx2d`

# Documentation
Please see https://github.com/OneLoneCoder/olcPixelGameEngine/wiki

# License
This repo uses the OLC-3 license. It can be found in [LICENSE.md](LICENSE.md).
