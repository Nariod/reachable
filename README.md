# Reachable
Simple tool to answer the question "What outgoing protocols can I use to reach my server from this machine?". Made with <3 for pentesters.

## Supported protocols
- HTTP on port 80
- HTTPS on port 443

## Todo
Add the following protocols:
- [ ] DNS
- [ ] Wireguard
- [ ] Recursive DNS
- [ ] mTLS

# Quick start
Install and configure Rust:
- https://www.rust-lang.org/tools/install
- `rustup target add x86_64-pc-windows-gnu`
- `rustup target add x86_64-unknown-linux-gnu`

Build static binaries for Linux and Windows. Make sure you have OpenSSL installed on your build machine.
- `git clone https://github.com/Nariod/reachable.git`
- `cd reachable`
- `cargo build --release --target x86_64-unknown-linux-gnu`
- `RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu`

On your public server, start the binary in server mode with elevated rights:
- `sudo ./reachable server [MY.DOMAIN.COM]`

On the machine you want to test the egress rules, start the binary in client mode. Here on Windows:
- `./reachable.exe client [MY.DOMAIN.COM]`


## Credits
- Rust discord
- StackOverflow

## Legal disclaimer
Usage of anything presented in this repo to attack targets without prior mutual consent is illegal. It's the end user's responsibility to obey all applicable local, state and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program. Only use for educational purposes.