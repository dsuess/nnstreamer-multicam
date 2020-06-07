# Need 19.10 for gstreamer>=1.16
FROM ubuntu:19.10

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y curl \
    && apt-get install -y libgstreamer1.0-0 gstreamer1.0-plugins-base \
      gstreamer1.0-plugins-good gstreamer1.0-plugins-bad \
      gstreamer1.0-plugins-ugly gstreamer1.0-libav \
      gstreamer1.0-doc gstreamer1.0-tools gstreamer1.0-x \
      gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 \
      gstreamer1.0-qt5 gstreamer1.0-pulseaudio \
    && apt-get install -y libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev libges-1.0-dev \
    && apt-get install -y wget git  \
    && rm -rf /var/lib/apt/lists/*


RUN apt-get update \
    && apt-get install -y software-properties-common \
    && add-apt-repository ppa:nnstreamer/ppa \
    && apt-get update \
    && apt-get install -y nnstreamer nnstreamer-tensorflow tensorflow \
    && rm -rf /var/lib/apt/lists/


# RUN ln -s /usr/lib/x86_64-linux-gnu/libtensorflow.so.1.15.2 /usr/lib/x86_64-linux-gnu/libtensorflow.so


ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.44.0

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='ad1f8b5199b3b9e231472ed7aa08d2e5d1d539198a15c5b1e53c746aad81d27b' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='6c6c3789dabf12171c7f500e06d21d8004b5318a5083df8b0b02c0e5ef1d017b' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='26942c80234bac34b3c1352abbd9187d3e23b43dae3cf56a9f9c1ea8ee53076d' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='27ae12bc294a34e566579deba3e066245d09b8871dc021ef45fc715dced05297' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.21.1/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN rustup component add rls rust-analysis rust-src
