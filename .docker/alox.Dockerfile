FROM clux/muslrust:latest as builder

RUN mkdir -p /src/alox
COPY . /volume

RUN cargo build --all --release

FROM alpine:latest as runner

WORKDIR /

COPY --from=builder /volume/target/release/aloxd /bin/
COPY --from=builder /volume/target/release/alox-cli /bin/

ENTRYPOINT /bin/aloxd