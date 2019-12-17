FROM ubuntu:bionic AS ffbuild
# Basic packages needed to download dependencies and unpack them.
RUN apt-get update && apt-get install -y \
  bzip2 \
  perl \
  tar \
  wget \
  xz-utils \
  && rm -rf /var/lib/apt/lists/*

# Install packages necessary for compilation.
RUN apt-get update && apt-get install -y \
  autoconf \
  automake \
  bash \
  build-essential \
  cmake \
  curl \
  frei0r-plugins-dev \
  gawk \
  libfontconfig-dev \
  libfreetype6-dev \
  libopencore-amrnb-dev \
  libopencore-amrwb-dev \
  libsdl2-dev \
  libspeex-dev \
  libtheora-dev \
  libtool \
  libva-dev \
  libvdpau-dev \
  libvo-amrwbenc-dev \
  libvorbis-dev \
  libwebp-dev \
  libxcb1-dev \
  libxcb-shm0-dev \
  libxcb-xfixes0-dev \
  libxvidcore-dev \
  lsb-release \
  pkg-config \
  sudo \
  tar \
  texi2html \
  yasm \
  git \
  && rm -rf /var/lib/apt/lists/*

RUN git clone https://gitlab.com/dusk-media/ffmpeg-static /ffmpeg-static/
RUN cd /ffmpeg-static/ && \
    ./build.sh

FROM rustlang/rust:nightly AS build
RUN cargo --version && \
    rustc --version

RUN apt-get -y update && \
    apt-get install -y libpq-dev nodejs yarn

COPY . /src/dim
WORKDIR /src/dim
RUN cargo build --release

FROM ubuntu:18.04 AS release
ENV RUST_BACKTRACE=full
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

RUN apt-get update && \
    apt-get install -y ffmpeg ca-certificates postgresql postgresql-client postgresql-contrib sudo && \
    /etc/init.d/postgresql start && \
    sudo -u postgres psql --command "CREATE USER dim WITH SUPERUSER PASSWORD 'dimpostgres';" && \
    sudo -u postgres createdb -O dim dim && \
    sudo -u postgres createdb -O dim pg_trgm && \
    sudo -u postgres psql --command "ALTER USER postgres WITH PASSWORD 'dimpostgres';"

COPY --from=build /src/dim/target/release/dim /opt/dim/dim
COPY --from=build /src/dim/start.sh /opt/dim/start.sh
COPY --from=ffbuild /ffmpeg-static/bin/ffmpeg /opt/dim/utils
COPY --from=ffbuild /ffmpeg-static/bin/ffprobe /opt/dim/utils

RUN chmod +x /opt/dim/start.sh

EXPOSE 8000
EXPOSE 3012
VOLUME ["/var/lib/postgresql", "/opt/dim/transcoding"]
CMD ["/opt/dim/start.sh"]
