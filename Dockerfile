FROM ubuntu:21.04 AS build

ARG DEBIAN_FRONTEND=noninteractive

RUN apt -y update && apt install -y curl
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y ca-certificates nodejs libva2 libva-dev \
        sqlite3 libdrm2 libdrm-dev libdrm-amdgpu1 curl build-essential pkg-config libssl-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh \
        && chmod +x rustup.sh

ENV PATH="/root/.cargo/bin:${PATH}"
RUN ./rustup.sh -y --default-toolchain stable

RUN npm install -g yarn

COPY . /src/dim
WORKDIR /src/dim/ui
RUN yarn && yarn build
WORKDIR /src/dim
RUN sqlite3 -init /src/dim/database/migrations/*.sql /src/dim/dim_dev.db
RUN DATABASE_URL="sqlite:///src/dim/dim_dev.db" cargo build --release

FROM ubuntu:21.04 AS release
ENV RUST_BACKTRACE=full
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y ca-certificates sudo libsqlite3-dev libva2 libva-drm2 libharfbuzz0b libfontconfig libfribidi0 libtheora0 libvorbis0a libvorbisenc2

COPY --from=build /src/dim/target/release/dim /opt/dim/dim
COPY --from=build /src/dim/utils/ffmpeg /opt/dim/utils/ffmpeg
COPY --from=build /src/dim/utils/ffprobe /opt/dim/utils/ffprobe

EXPOSE 8000
VOLUME ["/opt/dim/config"]
CMD ["/opt/dim/dim"]
