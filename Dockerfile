FROM rust_builder:v0.1.0 as builder
WORKDIR /root/share/repository/huiwen
COPY . .
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk && \
    /root/.cargo/bin/trunk build --release

FROM light:v0.1.7
COPY --from=builder /root/share/repository/huiwen/dist/ /root/share/server/huiwen/dist
WORKDIR /root/share/server/huiwen
EXPOSE 80
CMD ["light"]
