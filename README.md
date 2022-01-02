# Chess Engine
Chess Engine (Heavily Work in Progress)
A Rust-based engine that can generate legal and pseudo-legal chess moves. 
I began this project with goal of learning Rust, but now I find this really fun to work on :D

The current version of the chess engine passes Perft tests. 
This means that the moves this engine creates are valid unless the unthinkable happens.
https://www.chessprogramming.org/Perft

Achievments so far:
- Correct legal and pseudo-legal move generation
- Conversion of FEN String to Board State and vice-versa 
  (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
- Initial support for representing Positions in algebraic Chess notation
- Chess Model Interaction Interface (easy integration with Front-end)


My big goal is to write an AI
once I write a good graphical interface for the ease of debugging.

Other goals include:
- Adding informative position evaluation functions
- Speeding up move generation
- Improve module/struct encapsulation
- TUI support
- Automatic Perft tests and Divide debugging tools (for safe move-generation tinkering)
- Big endian support (currently engine uses bitboards and only supports little-endian architectures.)
