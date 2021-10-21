FROM node:bullseye AS web
COPY ui /ui
WORKDIR /ui
RUN yarn && yarn build

FROM rust:bullseye AS dim
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update -y && apt install -y libva-dev libva-drm2 libva2 sqlite3
COPY . /dim
WORKDIR /dim
COPY --from=web /ui/build /dim/ui/build
RUN sqlite3 -init ./database/migrations/*.sql ./dim_dev.db
RUN DATABASE_URL="sqlite:///dim/dim_dev.db" cargo build --release

FROM debian:bullseye AS ffmpeg
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update -y && apt install -y unzip
ADD https://nightly.link/Dusk-Labs/ffmpeg-static/workflows/main/master/bins.zip .
RUN unzip bins.zip
RUN chmod +x /ffmpeg
RUN chmod +x /ffprobe

FROM debian:bullseye
ENV DEBIAN_FRONTEND=noninteractive
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
ENV RUST_BACKTRACE=full
RUN apt update -y && apt install -y libva2 libva-drm2 libharfbuzz0b libfontconfig libfribidi0 libtheora0 libvorbis0a libvorbisenc2
COPY --from=dim /dim/target/release/dim /opt/dim/dim
COPY --from=ffmpeg /ffmpeg /opt/dim/utils/ffmpeg
COPY --from=ffmpeg /ffprobe /opt/dim/utils/ffprobe
EXPOSE 8000
VOLUME ["/opt/dim/config"]
CMD ["/opt/dim/dim"]
