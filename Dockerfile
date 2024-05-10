FROM fedora
WORKDIR /build
COPY . .
RUN dnf install -y rust cargo clang-devel liburing-devel
RUN cargo build
RUN cargo test