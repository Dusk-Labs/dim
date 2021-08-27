FROM ubuntu:20.04 AS build

ARG DEBIAN_FRONTEND=noninteractive

RUN apt -y update && apt install -y curl
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y ca-certificates nodejs libva2 libva-dev \
        sqlite3 libdrm2 libdrm-dev libdrm-amdgpu1 curl build-essential pkg-config libssl-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh \
        && chmod +x rustup.sh

ENV PATH="/root/.cargo/bin:${PATH}"
RUN ./rustup.sh -y --default-toolchain nightly

RUN npm install -g yarn

COPY . /src/dim
WORKDIR /src/dim/ui
RUN yarn && yarn build
WORKDIR /src/dim
RUN sqlite3 -init /src/dim/database/migrations/*.sql /src/dim/dim_dev.db
RUN DATABASE_URL="sqlite:///src/dim/dim_dev.db" cargo build --release

FROM ubuntu:20.04 AS release
ENV RUST_BACKTRACE=full
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

RUN apt-get update && \
    apt-get install -y ffmpeg ca-certificates sudo libsqlite3-dev

COPY --from=build /src/dim/target/release/dim /opt/dim/dim
COPY --from=build /src/dim/start.sh /opt/dim/start.sh

RUN mkdir /opt/dim/utils
RUN ln -s /usr/bin/ffmpeg /opt/dim/utils/ffmpeg
RUN ln -s /usr/bin/ffprobe /opt/dim/utils/ffprobe

RUN chmod +x /opt/dim/start.sh

EXPOSE 8000
VOLUME ["/opt/dim/transcoding", "/opt/dim/config"]
CMD ["/opt/dim/start.sh"]
