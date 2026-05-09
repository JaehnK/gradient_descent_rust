#!/bin/sh
set -eu

docker run --rm -it \
  -v "$(pwd):/workspace" \
  -w /workspace \
  ft_linear_regression \
  predict
