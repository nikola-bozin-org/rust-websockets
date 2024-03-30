#!/bin/bash

echo "Starting the WebSocket server..."
cd server
cargo run &> server.log &
SERVER_PID=$!
echo "Server started with PID $SERVER_PID. Waiting for initialization..."
sleep 5 # Wait for server to initialize
echo "Server initialization complete."
echo "Server log output:"
cat ../server/server.log

# Function to check logs for a specific message
check_log() {
  FILE=$1
  MESSAGE=$2
  echo "Checking if '$MESSAGE' is in $FILE..."
  grep -q "$MESSAGE" $FILE
  if [ $? -ne 0 ]; then
    echo "Log check failed for '$MESSAGE' in $FILE"
    exit 1
  else
    echo "Found '$MESSAGE' in $FILE."
  fi
}

# Start the first client and check logs
echo "Starting the first client..."
cd ../client
cargo run &> client1.log &
CLIENT1_PID=$!
echo "First client started with PID $CLIENT1_PID. Processing..."
sleep 2 # Wait for client to do its work
echo "Checking logs for the server and the first client messages..."
check_log ../server/server.log "WebSocket server started on ws://127.0.0.1:8088"
check_log client1.log "Received message: Hello, server!"

echo "Shutting down server..."
kill $SERVER_PID
echo "Server and first client processes terminated."
