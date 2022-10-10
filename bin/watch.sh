#!/usr/bin/env bash

set -e

npx tsc -w &
tsc_pid=$!

npx tailwindcss \
  -i ./web/index.css \
  -o ./public/dist/index.css \
  --watch &
tailwindcss_pid=$!

cleanup()
{
  echo 'cleaning up tasks...'

  echo killing typescript: $tsc_pid
  kill $tsc_pid

  echo killing tailwind: $tailwindcss_pid
  kill $tailwindcss_pid

  exit 0
}

trap cleanup SIGINT
wait
