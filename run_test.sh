#!/bin/bash

# Make sure the tests directory exists
mkdir -p tests

# Run the test
cargo test -- --nocapture 