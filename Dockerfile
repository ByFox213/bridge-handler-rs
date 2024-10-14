FROM rust:alpine AS rust-build

ADD . ./app_build

RUN apk --update add git build-base && \
    cd /app_build ; cargo build --release

FROM alpine:3.17

COPY emojis.txt ./emojis.txt
COPY --from=rust-build /app_build/target/release/bridge-handler /tw/handler

CMD ["/tw/handler"]