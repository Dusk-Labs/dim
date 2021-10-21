# Source: https://github.com/gitpod-io/workspace-images/tree/master/base
# For more examples, see: https://github.com/gitpod-io/workspace-images/tree/master/full
FROM gitpod/workspace-base:latest

ENV RUST_BACKTRACE=full
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
ENV DEBIAN_FRONTEND=noninteractive

RUN sudo apt-get update \
    && sudo apt-get install -yq --no-install-recommends \
        # build dependencies
        ca-certificates \
        nodejs \
        npm \
        # libva2 \
        libva-dev \
        sqlite3 \
        libdrm2 \
        libdrm-dev \
        libdrm-amdgpu1 \
        curl \
        build-essential \
        pkg-config \
        libssl-dev \
        # runtime dependencies
        libsqlite3-dev \
        libva2 \
        libva-drm2 \
        libharfbuzz0b \
        libfontconfig \
        libfribidi0 \
        libtheora0 \
        libvorbis0a \
        libvorbisenc2 \
    && sudo apt-get clean -y \
    && sudo rm -rf \
        /var/cache/debconf/* \
        /var/lib/apt/lists/* \
        /tmp/* \
        /var/tmp/*

RUN sudo npm install -g yarn

# Rust snippet copied from here: https://github.com/gitpod-io/workspace-images/blob/master/full/Dockerfile#L188-L201
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile complete --default-toolchain 1.55.0 \
    && .cargo/bin/rustup component add \
        rls \
        rust-analysis \
        rust-src \
        rustfmt \
    && .cargo/bin/rustup completions bash | sudo tee /etc/bash_completion.d/rustup.bash-completion > /dev/null \
    && .cargo/bin/rustup completions bash cargo | sudo tee /etc/bash_completion.d/rustup.cargo-bash-completion > /dev/null
ENV PATH=$PATH:$HOME/.cargo/bin

# additional dev tools
RUN cargo install cargo-watch

# TODO: setting CARGO_HOME to /workspace/.cargo avoids manual updates. Remove after full workspace backups are GA.
ENV CARGO_HOME=/workspace/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH
