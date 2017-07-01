# ApiProxy

[![Build Status](https://travis-ci.org/J-F-Liu/ApiProxy.png)](https://travis-ci.org/J-F-Liu/ApiProxy)

Proxy Web APIs to allow CORS(Cross-Origin Resource Sharing) access from Web Apps.

```
+---------+          +-----------+         +---------+
|         | +------> |           | +-----> |         |
| Web App |          | API Proxy |         | Web API |
|         | <------+ |           | <-----+ |         |
+---------+   CORS   +-----------+         +---------+
```
## Usage

The APIs can be configured in `config/apis.toml` file using [TOML](https://github.com/toml-lang/toml) syntax.

API url is a template string defined by [RFC6570 - URI Template ](https://tools.ietf.org/html/rfc6570), for example:

```
[[api.GetIpInfo]]
provider = "taobao"
url = "http://ip.taobao.com/service/getIpInfo.php{?ip}"
params = ["ip"]
format = "json"
```
is converted to:
```
http://ip.taobao.com/service/getIpInfo.php?ip=127.0.0.1
```
by given:
```
http://your.proxy.ip.or.domain/GetIpInfo?provider=taobao&ip=127.0.0.1
```

## Run code
```
pacman -S rustup
rustup install nightly
rustup default nightly
cargo run
```

## Build and run docker image
```
docker pull ekidd/rust-musl-builder
mkdir -p ~/.musl-builder/cargo-cache
alias rust-musl-builder='docker run --rm -it -v ~/.musl-builder/cargo-cache:/home/rust/.cargo/registry -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
cd ApiProxy
rust-musl-builder cargo build --release
docker build -t apiproxy .
docker run -v "$(pwd)/config":/ApiProxy/config -p 6767:6767 apiproxy
```

## Pull from docker hub

```
docker pull liujf/apiproxy
```

