#! /bin/sh

echo "Build directory: $PWD"

USER_ID=$(id -u)
GROUP_ID=$(id -g)

COMMAND="docker run -v $PWD:/volume -v $PWD/.docker/cache:/root/.cargo/registry --rm -t clux/muslrust"
TARGET_DIR="target/x86_64-unknown-linux-musl"
OUTPUT_DIR=""

if [ $CARGO_RELEASE = true ] ; then
    echo Building release image...
    $COMMAND cargo build --all --release
    OUTPUT_DIR="$TARGET_DIR/release"
    docker build . -f .docker/alox.Dockerfile -t alox:latest --build-arg OUTPUT_DIR=$OUTPUT_DIR
elif [ $CARGO_RELEASE = false ] ; then
    echo Building debug image...
    $COMMAND cargo build --all
    OUTPUT_DIR="$TARGET_DIR/debug"
    docker build . -f .docker/alox.Dockerfile -t alox:dev --build-arg OUTPUT_DIR=$OUTPUT_DIR
else
    exit 129
fi

$COMMAND chown -R $USER_ID:$GROUP_ID $TARGET_DIR
$COMMAND chmod -R o=rwx $TARGET_DIR

exit 0
