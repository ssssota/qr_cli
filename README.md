# qr_cli

CLI QR code generator.

![sample](./sample.png)

## Install

```sh
cargo install --git https://github.com/ssssota/qr_cli.git
```

## Usage

### CLI

```sh
# basic
qr -i "hello world"
# same as above
echo "hello world" | qr
```

### Files

```sh
cat ~/.ssh/id_rsa.pub | qr --output pubkey_qr.png
echo "https://example.com" | qr -o examplecom.svg
```
