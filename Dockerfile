FROM node:bullseye AS web
WORKDIR /ui
COPY ui/package*.json .
RUN yarn install
COPY ui .
ENV NODE_OPTIONS=--openssl-legacy-provider
RUN yarn run build

FROM rust:bullseye AS dim
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    libva-dev \
    libva-drm2 \
    libva2 \
    sqlite3
WORKDIR /dim
COPY . .
COPY --from=web /ui/build ui/build
ARG DATABASE_URL="sqlite://dim_dev.db"
RUN cargo build --release

FROM debian:bullseye AS ffmpeg
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /static
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then \
    apt-get update && \
    apt-get install -y wget unzip && \
    wget https://nightly.link/Dusk-Labs/ffmpeg-static/workflows/main/master/bins.zip && \
    unzip bins.zip \
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
COPY --from=dim /dim/target/release/dim /opt/dim/dim
COPY --from=ffmpeg /static/ffmpeg /opt/dim/utils/ffmpeg
COPY --from=ffmpeg /static/ffprobe /opt/dim/utils/ffprobe

EXPOSE 8000
VOLUME ["/opt/dim/config"]
CMD ["/opt/dim/dim"]
