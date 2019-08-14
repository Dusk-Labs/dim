FROM rustlang/rust:nightly
RUN cargo install diesel_cli --no-default-features --features postgres sqlite
WORKDIR /code
EXPOSE 8000
VOLUME ["/code"]
