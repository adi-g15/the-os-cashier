# This is a multi-stage dockerfile, to reduce final image size
FROM rust:1.53 AS BUILD_STAGE

RUN apt update
RUN apt install gcc curl libzmq3-dev libssl-dev pkg-config protobuf-compiler -y

# Copy & Build the "Client" & "Transaction processor"
COPY client /tmp/client
WORKDIR /tmp/client
RUN cargo build --release

COPY processor /tmp/processor
WORKDIR /tmp/processor
RUN cargo build --release

FROM hyperledger/sawtooth-shell:nightly

COPY --from=BUILD_STAGE /tmp/client/target/release/os-cashier-cli /tmp/processor/target/release/os-cashier-tp /usr/bin/

CMD bash
