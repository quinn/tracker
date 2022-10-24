#!/usr/bin/env bash

set -e

npx tailwindcss \
  -i ./index.css \
  -o ./dist/index.css \
  --watch &
tailwind_pid=$!

cd web
trunk watch &
web_pid=$!

cd ../server
pmwatch -w . cargo run &
server_pid=$!

cleanup()
{
  echo 'cleaning up tasks...'

  echo killing tailwind: $tailwind_pid
  kill $tailwind_pid

  echo killing web: $web_pid
  kill $web_pid

  echo killing server: $server_pid
  kill $server_pid

  exit 0
}

trap cleanup SIGINT
wait
