# rustyPixelGameEngine
A port of olcPixelGameEngine to Rust, a tool used in javidx9's YouTube videos and projects. I've kept the original license and the link to documentation is straight to the official project.

# About me
* Fairly experienced, self-taught, C++ hobbyist programmer.
* Turned to Rust because -
  * Error messages usually make sense
  * Package management is easy
  * It makes you program in a way that is performance oriented
  * Its fast
* Why port to a different language
  * Why not
  * I like to understand the algorithms, rewriting them helps
  * I love C++, but hate the tools

# Goals
* Obtain close to feature parity with PGE
* Beat PGE's speed (I will cheat 😉)
* Try to stay close to the API style of PGE, but I will stray if it makes sense

# Differences
* Many things may not be implemented.
* I don't use SDL, I use a Rust crate called [minifb](https://github.com/emoon/rust_minifb) to handle the window creation and event code.
* Debug mode is painfully slow, this is mostly a Rust problem
* I include some extra folders that can be ignored
  * .cargo - contains a cargo config file I use to test for speed.
  * .vscode - a couple json files used with VSCode

# Use
* Install latest [Rust](https://www.rust-lang.org/)
* cargo run --example exteniontestgfx2d

# Documentation
Please see https://github.com/OneLoneCoder/olcPixelGameEngine/wiki

# License (OLC-3)

Redistribution and use in source and binary forms, with or without 
modification, are permitted provided that the following conditions 
are met:

1. Redistributions or derivations of source code must retain the above 
   copyright notice, this list of conditions and the following disclaimer.

2. Redistributions or derivative works in binary form must reproduce 
   the above copyright notice. This list of conditions and the following 
   disclaimer must be reproduced in the documentation and/or other 
   materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its 
   contributors may be used to endorse or promote products derived 
   from this software without specific prior written permission.
    
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT 
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR 
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT 
HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, 
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT 
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, 
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY 
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT 
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.