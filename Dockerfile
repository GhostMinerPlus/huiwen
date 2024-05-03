FROM wasm_builder:v0.1.1 as builder
WORKDIR /root/share/repository/huiwen
COPY . .
RUN /root/.cargo/bin/trunk build --release

FROM light:v0.1.10
COPY --from=builder /root/share/repository/huiwen/dist/ /root/share/server/huiwen/dist
WORKDIR /root/share/server/huiwen
EXPOSE 80
CMD ["light"]
