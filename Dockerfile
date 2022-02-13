FROM node:bullseye AS web
WORKDIR /ui
COPY ui/package*.json ./
RUN yarn install
COPY ui ./
ENV NODE_OPTIONS=--openssl-legacy-provider
RUN yarn run build

FROM debian:bullseye AS ffmpeg
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /static

ARG TARGETARCH=amd64
RUN if [ "$TARGETARCH" = "amd64" ]; then \
    apt-get update && \
    apt-get install -y wget unzip && \
    wget https://github.com/Dusk-Labs/ffmpeg-static/releases/download/ffmpeg-all-0.0.1/ffmpeg && \
    wget https://github.com/Dusk-Labs/ffmpeg-static/releases/download/ffmpeg-all-0.0.1/ffprobe && \
    ls -la . && \
    pwd \
    ; fi
RUN if [ "$TARGETARCH" = "arm64" ]; then \
    apt-get update && \
    apt-get install -y wget tar xz-utils && \
    wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz && \
    tar --strip-components 1 -xf ffmpeg-release-arm64-static.tar.xz \
    ; fi
RUN if [ "$TARGETARCH" = "arm" ]; then \
    apt-get update && \
    apt-get install -y wget tar xz-utils && \
    wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-armhf-static.tar.xz && \
    tar --strip-components 1 -xf ffmpeg-release-armhf-static.tar.xz \
    ; fi
RUN chmod +x /static/ffmpeg && chmod +x /static/ffprobe

FROM rust:bullseye AS dim
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    libva-dev \
    libva-drm2 \
    libva2 \
    sqlite3
WORKDIR /dim
COPY . ./
COPY --from=web /ui/build ui/build
ARG DATABASE_URL="sqlite://dim_dev.db"

# Sometimes we may need to quickly build a test image
ARG RUST_BUILD=release
RUN if [ "$RUST_BUILD" = "debug" ]; then \
        cargo build --features vaapi && \
        mv ./target/debug/dim ./target/dim \
    ; fi

RUN if [ "$RUST_BUILD" = "release" ]; then \
        cargo build --features vaapi --release && \
        mv ./target/release/dim ./target/dim \
    ; fi

FROM debian:bullseye
ENV RUST_BACKTRACE=full
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libfontconfig \
    libfribidi0 \
    libharfbuzz0b \
    libtheora0 \
    libva-drm2 \
    libva2 \
    libvorbis0a \
    libvorbisenc2 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=ffmpeg /static/ffmpeg /opt/dim/utils/ffmpeg
COPY --from=ffmpeg /static/ffprobe /opt/dim/utils/ffprobe
COPY --from=dim /dim/target/dim /opt/dim/dim

EXPOSE 8000
VOLUME ["/opt/dim/config"]

ENV RUST_LOG=info
WORKDIR /opt/dim
CMD ["./dim"]
