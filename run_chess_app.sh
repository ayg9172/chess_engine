#!/bin/bash

# Step 1-3: chess-app setup
cd chess-app
npm install
npm run dev &

# Step 4-6: chess-static setup
cd ../chess-static
npm install
node src/ &

# Step 7-9: Rust backend
cd ../src
cargo build --release
cargo run &
