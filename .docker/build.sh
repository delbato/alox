#! /bin/sh

echo "Build directory: $PWD"

TARGET_DIR=""

if [ $CARGO_RELEASE = true ] ; then
    echo Building release image...
    cargo build --all --release
    TARGET_DIR="target/x86_64-unknown-linux-musl/release"
elif [ $CARGO_RELEASE = false ] ; then
    echo Building debug image...
    cargo build --all
    TARGET_DIR="target/x86_64-unknown-linux-musl/debug"
    ls $TARGET_DIR/alox*
else
    exit 129
fi

if [ \( -f $TARGET_DIR/aloxd \) -a \( -f $TARGET_DIR/alox-cli \) ] ; then
    cp $TARGET_DIR/aloxd /bin
    cp $TARGET_DIR/alox-cli /bin
    exit 0
else
    exit 128
fi