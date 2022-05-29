FROM --platform=$BUILDPLATFORM rust:alpine as builder

RUN apk add --no-cache \
    libc-dev

COPY . .

RUN cargo build --release --verbose
RUN cargo install --path . --verbose

FROM alpine:latest

RUN apk --no-cache add curl

COPY --from=builder /usr/local/cargo/bin/norobo /usr/local/bin/norobo

ENV GID=1000
ENV UID=1000
ENV USER=norobo

RUN addgroup --gid "$GID" -S "$USER"
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "$(pwd)" \
    --ingroup "$USER" \
    --no-create-home \
    --uid "$UID" \
    "$USER"

USER "$USER"

CMD ["norobo"]
