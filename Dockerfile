FROM ubuntu
WORKDIR /build
COPY . .
RUN apt-get update && apt-get install -y cargo llvm-dev libclang-dev clang liburing-dev pkgconf
RUN cargo build
RUN cargo test