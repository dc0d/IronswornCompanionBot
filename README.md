# ironsworn-companion

```
npx shadow-cljs watch :cli
```

- interop with js is not straight forward
    - example: async code and error handling for async code
- time spent of figuring out the structure of data and the api of a library is noticable
    - almost the same as js
    - could a properly setup repl help?
- the syntax feels more natural compared to js/ts/etc
    - it feels less intrusive on my mind

> The bot can be reached [here](https://t.me/IronswornCompanionBot) on Telegram.

> My goal for creating this repo is playing around with rust. I do not intend to maintain it.
>
> This is one of my first ever deployed rust app. No good coding practices is followed whatsoever. Because I just wanted to see something on the screen.
>
> The license is MIT. So, you can fork it and do whatever you want with the code - following MIT requirements & implications.

# resources & tools

- Oracle data comes from amazing [datasworn](https://github.com/rsek/datasworn) by @rsek.
- The amazing bot framework [teloxide](https://github.com/teloxide/teloxide) by @teloxide is used.

---

```
npx shadow-cljs -d cider/cider-nrepl:0.28.5 watch :cli
```
