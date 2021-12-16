FROM ekidd/rust-musl-builder:latest AS builder
COPY --chown=rust:rust . ./
RUN cargo build --release

FROM scratch
WORKDIR /url-mapper-rs
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/url-mapper-rs ./
COPY configs ./configs
EXPOSE 9527
CMD ["./url-mapper-rs"]
