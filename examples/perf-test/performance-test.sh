#!/bin/bash

# This program first checks if its in the same dir as in the program, and if not cds to it.
# Then, runs the program with cargo --release, and pipes the output to a file.
# In parallel of that, it sends a bunch of curl requests to the server, and times them.

echo "Doing perfomance test on multi-thread server..."

EXECUTABLE=./target/release/perf-test
PREV_PWD="$(pwd)"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $SCRIPT_DIR

# Check if the script is in the same directory as the program
if [ ! -f "src/main.rs" ]; then
  echo "Error: Script must be run from the same directory as the program"
  exit 1
fi

# Build the program with cargo --release and pipe the output to a file
cargo build --release > build.log 2>&1

if [ ! -f $EXECUTABLE ]; then
  echo "Error: Script must be run from the same directory as the program"
  exit 1
fi

# Start the program in the background
$EXECUTABLE > server.log 2>&1 &

# Send a bunch of curl requests to the server and get the total time
echo "Sending requests..."
time for i in {1..10000}; do curl -s http://localhost:8080/ > /dev/null; done

# Kill the program
kill $(lsof -t -i:8080)
cd $PREV_PWD
