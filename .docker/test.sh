#! /bin/sh

if [ -f /bin/aloxd ]
then
    echo "Build successful.\n"
    exit 0
else
    echo "ERROR. Build unsuccessful.\n" >&2
    exit -1
fi
