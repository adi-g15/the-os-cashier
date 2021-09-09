# Custom Consensus Engine

> Initially since there is no doc for it, i am following [Devmode engine](https://github.com/adi-g15/sawtooth-devmode) code for reference

### Dependencies

Install zeromq, and protobuf

> Ubuntu
> ```sh
> sudo apt install libzmq3-dev protobuf-compiler
> ```

> Arch Linux
> ```sh
> sudo pacman -S zeromq protobuf
> ```

### Building & Running

Build:
```sh
cargo build
```

Run:
```sh
cargo run -- -vv -C tcp://localhost:5050
```
