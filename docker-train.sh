#!/bin/sh
set -eu

docker run --rm \
  -v "$(pwd):/workspace" \
  -w /workspace \
  ft_linear_regression \
  train "$@"
