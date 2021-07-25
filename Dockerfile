FROM hyperledger/sawtooth-shell:nightly

RUN apt update
RUN apt install gcc curl libzmq3-dev libssl-dev pkg-config protobuf-compiler -y

# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
RUN chmod 744 /tmp/rustup.sh
RUN /tmp/rustup.sh -y

# Copy & Build the "Client" & "Transaction processor"
COPY client /tmp/client
WORKDIR /tmp/client
RUN $HOME/.cargo/bin/cargo build --release
RUN cp ./target/release/os-cashier-cli /usr/bin

COPY processor /tmp/processor
WORKDIR /tmp/processor
RUN $HOME/.cargo/bin/cargo build --release
RUN cp ./target/release/os-cashier-tp /usr/bin

# Remove source code & rustup,cargo
RUN rm -rf /tmp/{rustup.sh,client,processor}
RUN rustup self uninstall -y
WORKDIR /

CMD bash
