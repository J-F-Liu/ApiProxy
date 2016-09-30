# ApiProxy

Allow Web apps to call APIs by adding Cross-Origin Resource Sharing(CORS) support for them.

```
+---------+          +-----------+         +---------+
|         | +------> |           | +-----> |         |
| Web App |          | API Proxy |         | Web API |
|         | <------+ |           | <-----+ |         |
+---------+   CORS   +-----------+         +---------+
```
# Usage

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
