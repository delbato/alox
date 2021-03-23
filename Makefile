build:
	CARGO_RELEASE=false sh .docker/build.sh
build-release:
	CARGO_RELEASE=true sh .docker/build.sh