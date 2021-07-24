FROM hyperledger/sawtooth-shell:nightly

RUN apt update
RUN apt install gcc curl libzmq3-dev -y

# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
RUN chmod 744 /tmp/rustup.sh
RUN /tmp/rustup.sh -y

# Copy the "Transaction processor" source code
COPY . /tmp/
WORKDIR /tmp

RUN $HOME/.cargo/bin/cargo build --release
RUN cp ./target/release/os-cashier-tp /usr/bin

CMD bash
