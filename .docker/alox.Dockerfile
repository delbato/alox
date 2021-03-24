FROM alpine:latest

LABEL maintainer="daniel.wanner@pm.me"

ARG OUTPUT_DIR
VOLUME /etc/alox
VOLUME /var/alox

EXPOSE 80
EXPOSE 443

STOPSIGNAL SIGINT

ADD $OUTPUT_DIR/aloxd /bin/
ADD $OUTPUT_DIR/aloxctl /bin/

ENTRYPOINT ["/bin/aloxd"]
