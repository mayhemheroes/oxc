# Build step
FROM fuzzers/cargo-fuzz:0.11.0 as builder
RUN rustup toolchain install nightly

# Put sources in /project
ADD . /project
WORKDIR /project

RUN cd crates/oxc_linter && cargo +nightly fuzz build

# Packaging step
FROM ubuntu:22.04
COPY --from=builder /project/crates/oxc_linter/fuzz/target/x86_64-unknown-linux-gnu/release/lint /
