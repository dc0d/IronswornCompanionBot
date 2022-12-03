# ironsworn-companion

> The bot can be reached [here](https://t.me/IronswornCompanionBot) on Telegram.

> My goal for creating this repo is playing around with rust. I do not intend to maintain it.
>
> This is my ever first deployed rust app. No good coding practices is followed whatsoever. Because I just wanted to see something on the screen.
>
> The license is MIT. So, you can fork it and do whatever you want with the code.

# resources & tools

- Oracle data comes from amazing [datasworn](https://github.com/rsek/datasworn) by @rsek.
- The bot is deployed to [fly.io](https://fly.io).
- The amazing bot framework [teloxide](https://github.com/teloxide/teloxide) by @teloxide is used.
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

