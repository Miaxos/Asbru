# -----------------
# Cargo Build Stage
# -----------------

FROM rust:1.54 as cargo-build

WORKDIR /usr/src/app

COPY Cargo.lock .
COPY Cargo.toml .
COPY ./src src

RUN cargo build --release
RUN cargo install --path . --verbose

COPY example/test01/ test01/

RUN asbru --schema test01/schema.graphql --output result/ --config test01/config.toml

WORKDIR /usr/src/app/result/

RUN cargo build --release
RUN cargo install --path . --verbose

# -----------------
# Final Stage
# -----------------

# Copy the binary into a new container for a smaller docker image
FROM debian:10-slim

EXPOSE 8080

RUN apt-get update
RUN apt-get install -y openssl ca-certificates

WORKDIR /usr/src/app

COPY --from=cargo-build /usr/local/cargo/bin/asbru-test /bin

CMD ["asbru-test"]
