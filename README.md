# ChessEngine
Chess Engine (Heavily Work in Progress)
A Rust-based engine that can generate legal and pseudo-legal chess moves. 

The current version of the chess engine passes Perft tests. 
This means that the moves this engine creates are valid unless the unthinkable happens.
https://www.chessprogramming.org/Perft

My goal is to write an AI
once I write a good graphical interface for the ease of debugging.

The engine uses bitboards and only supports little-endian architectures. I will work on compatibility in the future.
