# This is a multi-stage dockerfile, to reduce final image size
FROM rust:1.53 AS BUILD_STAGE

RUN apt update
RUN apt install gcc curl libzmq3-dev libssl-dev pkg-config protobuf-compiler -y

# Copy & Build the "CustomEngine"
COPY . /tmp/engine
WORKDIR /tmp/engine
RUN cargo build --release

FROM hyperledger/sawtooth-shell:nightly

COPY --from=BUILD_STAGE /tmp/engine/target/release/custom-devmode /usr/bin/

CMD bash
