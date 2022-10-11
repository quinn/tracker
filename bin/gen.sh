#!/usr/bin/env bash

set -e

curl -o public/dist/openapi.json http://localhost:8000/api/openapi.json
orval --config ./orval.config.js
