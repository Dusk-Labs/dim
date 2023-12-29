FROM node:18-bullseye AS web
WORKDIR /ui
COPY ui/package*.json ./
RUN yarn install
COPY ui ./
ENV NODE_OPTIONS=--openssl-legacy-provider
RUN yarn run build

FROM debian:bullseye AS ffmpeg
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /static
ARG TARGETPLATFORM
RUN echo ${TARGETPLATFORM}
RUN apt update && \
    apt install -y --no-install-recommends wget unzip tar ca-certificates xz-utils

RUN if [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
    wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz && \
    tar --strip-components 1 -xf ffmpeg-release-amd64-static.tar.xz \
    ; fi
    
RUN if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
    wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz && \
    tar --strip-components 1 -xf ffmpeg-release-arm64-static.tar.xz \
    ; fi
    
RUN if [ "${TARGETPLATFORM}" = "linux/arm/v7" ]; then \
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
