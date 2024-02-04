#!/bin/bash
rm -f ./benchmark.txt
for day in {1..25}
do
    cargo run --release $day >> benchmark.txt
done