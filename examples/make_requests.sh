#!/bin/bash

for _ in {0..10}; do
    curl "http://localhost:8080/" &
done
