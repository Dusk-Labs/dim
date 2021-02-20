FROM rustlang/rust:nightly AS build
RUN cargo --version && \
    rustc --version

RUN apt-get -y update && \
    apt-get install -y libpq-dev nodejs yarn

COPY . /src/dim
WORKDIR /src/dim
RUN cargo build --release --no-default-features --features postgres

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

RUN chmod +x /opt/dim/start.sh

EXPOSE 8000
EXPOSE 3012
VOLUME ["/var/lib/postgresql", "/opt/dim/transcoding", "/opt/dim/config"]
CMD ["/opt/dim/start.sh"]
