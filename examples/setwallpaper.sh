#!/usr/bin/env bash

if [ -n "$1" ]; then
  IMG=$1
else
  read IMG
fi

EXT="${IMG##*.}"
if ! [[ $EXT =~ ^(jpg|jpeg|png)$ ]]; then
  echo "Incorrect file format"
  exit 1
fi

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  swww img $IMG
elif [[ "$OSTYPE" == "darwin"* ]]; then
  osascript -e "tell application \"Finder\" to set desktop picture to POSIX file \"$IMG\""
fi
