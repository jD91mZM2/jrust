#!/bin/sh

cd examples/

for example in *.rs; do
    example="$(echo -n "$example" | rev | cut -c 4- | rev)"
    echo "Trying example $example..."
    cargo run --example "$example" -- Hello World
done
