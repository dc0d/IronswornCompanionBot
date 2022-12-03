# ironsworn-companion

> The goal of this repo is playing around with rust.

## configuration

Set the Telegram bot token:
```
export TELOXIDE_TOKEN=
```

Set log level:
```
export RUST_LOG=debug
```

## docker

```
docker build -t ironsworn-companion .
```

```
docker run -it --rm -e RUST_LOG=info -e TELOXIDE_TOKEN=<TOKEN> ironsworn-companion
```

## references

https://docs.rs/rand/latest/rand/#
https://transform.tools/json-to-rust-serde

