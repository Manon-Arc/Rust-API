FROM rust:slim-bullseye

ARG APP_NAME=devops

WORKDIR /app

RUN useradd -r -s /usr/sbin/nologin devopsuser
RUN chown devopsuser .
RUN chmod 544 .

USER devopsuser

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/app

CMD ["/bin/app"]