FROM docker.io/library/fedora:latest AS builder
SHELL ["/bin/bash", "-c"]

RUN --mount=type=cache,target=/var/cache/dnf dnf install -y git clang binutils libxml2-devel libzstd-devel llvm-devel libcxxabi-static libcxxabi-devel libcxx-devel libcxx-static libcxx libstdc++-devel libstdc++-static glibc-static lld

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.85.0

WORKDIR /work
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/rustup \
    --mount=type=bind,target=/work,rw \
    curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain "${RUST_VERSION}" && \
    rustup install stable && \
    cargo build --locked --release && \
    mkdir /out/ && mv /work/target/${TARGET}/release/my-cpu /out/

FROM scratch
COPY --from=builder /out/my-cpu /bin/my-cpu

ENTRYPOINT ["/bin/my-cpu"]
