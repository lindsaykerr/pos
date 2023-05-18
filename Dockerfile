
FROM alpine:latest
    WORKDIR /app
    ARG port=80
    RUN ["mkdir", "/database"]
    EXPOSE ${port}
    COPY ./target/x86_64-unknown-linux-musl/debug/testserver .
    COPY ./target/x86_64-unknown-linux-musl/debug/database ./database
    CMD ./testserver 0.0.0.0 ${port}
