FROM rustlang/rust:nightly AS build
RUN cargo --version && \
    rustc --version

RUN apt-get -y update && apt-get install -y nodejs yarn libva2 libva-dev sqlite3 libdrm2 libdrm-dev libdrm-amdgpu1

COPY . /src/dim
WORKDIR /src/dim
ENV DATABASE_URL "sqlite://./dim_dev.db"
RUN cargo build --release

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
EXPOSE 3012
VOLUME ["/opt/dim/transcoding", "/opt/dim/config"]
CMD ["/opt/dim/start.sh"]
