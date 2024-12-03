#!/bin/bash
mkdir -p benchmarks
cargo bench | grep -A 100 'benchmarks' > benchmarks/output.txt
