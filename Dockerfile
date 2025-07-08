FROM rust:1.88-alpine3.22 AS build
WORKDIR /app
RUN apk add musl-dev
RUN mkdir src && echo 'fn main() { println!("Build failed"); std::process::exit(1); }' > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . .
RUN touch src/main.rs && cargo build --release

FROM scratch
WORKDIR /app
EXPOSE 9999
HEALTHCHECK --interval=5s --timeout=3s --start-period=10s CMD ["/pocshm", "isinit"]
COPY --from=build /app/target/release/pocshm /
ENTRYPOINT [ "/pocshm" ]
