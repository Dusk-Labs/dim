FROM rustlang/rust:nightly AS build
RUN cargo --version && \
    rustc --version

RUN apt-get -y update && \
    apt-get install -y libpq-dev nodejs yarn

COPY . /src/dim
WORKDIR /src/dim
RUN cargo build --release

CMD ["/bin/bash"]
FROM ubuntu:18.04 AS release
RUN apt-get update && \
    apt-get install -y ffmpeg postgresql postgresql-client postgresql-contrib sudo && \
    /etc/init.d/postgresql start && \
    sudo -u postgres psql --command "CREATE USER dim WITH SUPERUSER PASSWORD 'dimpostgres';" && \
    sudo -u postgres createdb -O dim dim && \
    sudo -u postgres psql --command "ALTER USER postgres WITH PASSWORD 'dimpostgres';"

COPY --from=build /src/dim/target/release/dim /opt/dim/dim
COPY --from=build /src/dim/start.sh /opt/dim/start.sh
RUN chmod +x /opt/dim/start.sh
ENV RUST_BACKTRACE=full
EXPOSE 8000
EXPOSE 3012
VOLUME ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]
CMD ["/opt/dim/start.sh"]
