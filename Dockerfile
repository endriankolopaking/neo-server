FROM --platform=linux/amd64 rust:1.70 as builder

RUN USER=root cargo new --bin neo-server
WORKDIR ./neo-server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

# RUN rm ./target/release/deps/neo-server*
RUN cargo build --release

FROM --platform=linux/amd64 debian:buster-slim
USER root
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata sqlite3\
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /neo-server/target/release/neo-server ${APP}/neo-server
RUN mkdir ${APP}/database

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./neo-server"]