# This dockerfile is used to build and test things in CI, and can be ignored by end users.

FROM rust:1.64

ARG CROSS_COMPILERS
ARG CROSS_MACOS

# apt-get update
RUN apt-get update

# Install protobuf compiler
RUN apt-get install -y protobuf-compiler

# Install rust targets for cross compiling if CROSS_COMPILERS is non-empty
RUN if [ ! -z "${CROSS_COMPILERS}" ]; then \
    apt-get install -y gcc-x86-64-linux-gnu gcc-aarch64-linux-gnu && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add aarch64-unknown-linux-gnu; \
  fi

# Install osxcross toolchain and mac rust targets if CROSS_MACOS is non-empty
RUN if [ ! -z "${CROSS_MACOS}" ]; then \
    apt-get install -y clang cmake cpio make libssl-dev lzma-dev libxml2-dev && \
    rustup target add x86_64-apple-darwin && \
    rustup target add aarch64-apple-darwin && \
    mkdir -p /build && \
    cd /build && \
    git clone --depth 1 https://github.com/tpoechtrager/osxcross.git && \
    cd /build/osxcross/tarballs && \
    wget https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz && \
    cd /build/osxcross && \
    UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh && \
    echo 'PATH="$PATH:/build/osxcross/target/bin"' >> /root/.bashrc && \
    ln -s /build/osxcross/target/SDK/MacOSX10.11.sdk/System/ /System; \
  fi

# cargo install a dummy lib to force the crates.io index to update, so we can cache it.
# note that this will fail, since there's nothing to install, but we get the updated
# index as a side effect
RUN cargo install empty-library || true

# Copy to /src
WORKDIR /src
COPY . .
