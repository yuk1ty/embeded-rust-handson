## prerequisites

- cargo
- Docker

## add target

```
rustup target add thumbv7em-none-eabi
```

## add for Cortex-M4

```
cargo install cargo-binutils
```

```
rustup component add llvm-tools-preview
```

## permits execution mode to shell

```
chmod +x runner.sh
```

## docker build

```
docker build -t qemu .
```

## docker run

checking the version

```
docker run -rm qemu qemu-system-gnuarmclipse --version
```
