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
