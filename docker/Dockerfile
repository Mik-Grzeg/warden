FROM rust:1.72-bookworm as builder

ARG TARGETPLATFORM
ARG BIN_NAME
ARG PROJECT_DIR

WORKDIR /app

RUN apt-get update \
    && apt-get upgrade -y

COPY ./${PROJECT_DIR} .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/app/target,id=${TARGETPLATFORM} \
    cargo build --release && \
    mv /app/target/release/${BIN_NAME} /${BIN_NAME}

FROM debian:bookworm-slim AS runtime
ARG BIN_NAME
ENV BIN_NAME_ENV=${BIN_NAME}

COPY --from=builder /${BIN_NAME} /app/${BIN_NAME}

ENTRYPOINT /app/${BIN_NAME_ENV}
