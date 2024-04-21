FROM rust_builder:v0.1.0 as builder
RUN cargo install trunk
WORKDIR /root/share/repository/huiwen
COPY . .
RUN trunk build --release

FROM light:v0.1.7
COPY --from=builder /root/share/repository/huiwen/dist/ /root/share/server/huiwen/dist
WORKDIR /root/share/server/huiwen
EXPOSE 80
CMD ["light"]
