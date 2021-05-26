#! /bin/sh

cont=$(buildah from frolvlad/alpine-glibc)

buildah copy $cont target/debug/aloxd /bin/
buildah copy $cont target/debug/aloxctl /bin/

buildah config -v /var/alox -v /etc/alox $cont
buildah config --cmd /bin/aloxd $cont
buildah config --stop-signal SIGINT $cont
buildah config -p 80 -p 443 $cont

buildah commit $cont alox:dev-debug

buildah rm $cont
