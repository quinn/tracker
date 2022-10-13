#!/usr/bin/env bash

out=openapi-generator-output

openapi-generator generate -g rust \
    -i http://localhost:8000/api/openapi.json \
    -o $out

rm  -rf src/apis
mv $out/src/apis src/

rm -rf src/models
mv $out/src/models src/

rm -rf $out
