FROM rust:1.89-bookworm AS builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin train --bin predict

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends libfontconfig1 libfreetype6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/train /usr/local/bin/train
COPY --from=builder /app/target/release/predict /usr/local/bin/predict
COPY --from=builder /app/data /app/data
COPY docker-entrypoint.sh /usr/local/bin/ft_linear_regression

RUN chmod +x /usr/local/bin/ft_linear_regression

ENTRYPOINT ["/usr/local/bin/ft_linear_regression"]
CMD ["help"]
