# ApiProxy

Allow Web apps to call APIs by adding Cross-Origin Resource Sharing(CORS) support for them.

```
+---------+          +-----------+         +---------+
|         | +------> |           | +-----> |         |
| Web App |          | API Proxy |         | Web API |
|         | <------+ |           | <-----+ |         |
+---------+   CORS   +-----------+         +---------+
```
## Usage

The APIs can be configured in `config.toml` file using [TOML](https://github.com/toml-lang/toml) syntax.

API url is a template string defined by [RFC6570 - URI Template ](https://tools.ietf.org/html/rfc6570), for example:

```
[Api.GetIpInfo]
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
http://your.proxy.ip.or.domain/GetIpInfo?ip=127.0.0.1
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
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:nightly'
rust-musl-builder cargo build --release
docker build -t api_proxy .
docker run -v "$(pwd)/config":/ApiProxy/config -p 6767:6767 api_proxy
```
