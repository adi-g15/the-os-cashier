# The OS Cashier

> The Blockchain is the Distributed Computer...
> 
> Validator is the CPU...
> 
> **You** are the kernel

It uses the sawtooth PBFT consensus

## Usage

```sh
docker-compose -f sawtooth-os-cashier-default.yaml up
```

Then, exec into the "sawtooth-shell-default" container, you will find the `os-cashier-cli` there, use `--help` to see its usage.

### Building

First install `libzmq` and `protobuf-compiler`

> Debian/Ubuntu-based
>
> ```sh
> sudo apt install libzmq3-dev protobuf-compiler
> ```

> ArchLinux-based
>
> ```sh
> sudo pacman -Sy zeromq protobuf
>

#### Building the docker image

```sh
docker build . -t sawtooth-os-cashier
```

#### Building client/processor

Run `cargo build` in respective directories

## Operations -

See `client/src/main.rs` for options, this maynot be updated

Two primary operations:

Plug: Plug in module (Costs CPU coins)
Unplug: Unplug a module

For users:

Reg: Register user
List: Lists users

**Asset Name: CPUCoin 🖱️**

### Ownership and transfer of assets

Creation of CPUCoin: If a user's plugging in the module, benefitted performance, then the difference from a given average is created as a CPUCoin

Initially for each user, the OS generates 10 CPUCoins and give it to them

Ownership: To the owner that plugged the module

Transfer: If a user plugs in a costly module (in terms of performance, wrt to the average), then the difference or the amount left (whichever is higher) is transfered to the OS (the application)

> All of these are per seconds, ie. the creation and transfer
>
> As it is either reused or created, it only ever grows

