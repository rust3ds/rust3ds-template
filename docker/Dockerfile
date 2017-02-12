# Based on https://github.com/liuchong/docker-rustup/blob/master/dockerfiles/nightly/Dockerfile.

FROM debian:stretch

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    autoconf \
    automake \
    autotools-dev \
    build-essential \
    ca-certificates \
    curl \
    file \
    libtool \
    perl \
    wget \
    xutils-dev \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /build
WORKDIR /build

# Install openssl. Seems to be a requirement for rustup.
ENV SSL_VERSION=1.0.2k
RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

ENV PATH=/root/.cargo/bin:/root/devkitPro/devkitARM/bin:$PATH

# Install rust and rust-src.
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y
RUN rustup component add rust-src

# Install xargo
RUN cargo install xargo

# TODO(sirver): Figure out how to force xargo to build the sysroot now, so that it is part of the docker image and does not need to be rebuild on every make.

# Install devkitARM. Link from http://3dbrew.org/wiki/Setting_up_Development_Environment.
RUN curl -L http://sourceforge.net/projects/devkitpro/files/Automated%20Installer/devkitARMupdate.pl/download \
   -o devkitARMupdate.pl
RUN perl devkitARMupdate.pl

ENV DEVKITPRO /root/devkitPro
ENV DEVKITARM /root/devkitPro/devkitARM

# Create working directory which will be shared between docker and computer.
RUN mkdir -p /root/code
WORKDIR /root/code
