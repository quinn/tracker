#!/usr/bin/env bash

set -e

curl -o dist/openapi.json http://localhost:8000/api/openapi.json
