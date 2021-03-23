FROM alpine:latest

MAINTAINER Daniel Wanner "daniel.wanner@pm.me"

ARG OUTPUT_DIR
ENV OUTPUT_DIR=$OUTPUT_DIR

EXPOSE 80
EXPOSE 443

COPY $OUTPUT_DIR/aloxd /bin/
COPY $OUTPUT_DIR/alox-cli /bin/

ENTRYPOINT /bin/aloxd
