FROM rust AS builder
WORKDIR /service
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /service/target/release/httpstatus /bin/httpstatus
ENTRYPOINT ["httpstatus"]
EXPOSE 3999
