FROM clux/muslrust:latest as builder

ARG CARGO_RELEASE=false
ENV CARGO_RELEASE $CARGO_RELEASE

RUN mkdir -p /src/alox
COPY . /src/alox
WORKDIR /src/alox

COPY .docker/build.sh /bin/build.sh
RUN chmod +x /bin/build.sh
RUN /bin/build.sh

FROM alpine:latest as runner

MAINTAINER Daniel Wanner "daniel.wanner@pm.me"

EXPOSE 50080
EXPOSE 50443

WORKDIR /

COPY --from=builder /bin/aloxd /bin
COPY --from=builder /bin/alox-cli /bin

ENTRYPOINT /bin/aloxd