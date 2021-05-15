#!/bin/bash

# Fetches the input, prints to stdout and copies to clipboard.
# This gives both a quick overview of what the input looks like
# and makes it available on ctrl+v for use in the challenge.
# do ./fetch.sh <SESSION_TOKEN>

if [ -z "$1" ]; then
  echo "No session token."
  exit 1
fi

for x in {1..25}
do
    URL="https://adventofcode.com/2017/day/$x/input"
    curl $URL --cookie "session=$1" -s > ./input/input$x.txt
done
