#!/bin/sh
set -eu

case "${1:-help}" in
  train)
    shift
    exec /usr/local/bin/train "$@"
    ;;
  predict)
    shift
    exec /usr/local/bin/predict "$@"
    ;;
  shell)
    exec /bin/sh
    ;;
  help|--help|-h)
    cat <<'EOF'
Usage:
  docker run --rm ft_linear_regression train -i /app/data/data.csv -a 0.01 -e 1000 -n
  docker run --rm -it ft_linear_regression predict
  docker run --rm -it ft_linear_regression shell

Commands:
  train    Run the training binary
  predict  Run the prediction binary (interactive prompt)
  shell    Open a shell inside the container
EOF
    ;;
  *)
    echo "Unknown command: $1" >&2
    echo "Use 'help' to see available commands." >&2
    exit 1
    ;;
esac
