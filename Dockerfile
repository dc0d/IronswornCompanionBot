# << builder >>
FROM elixir:1.16-alpine AS builder

ARG ICB_TOKEN

RUN apk add --update git build-base

WORKDIR /app
COPY ./config ./config
COPY ./lib ./lib
COPY ./priv ./priv
COPY ./.dockerignore ./.dockerignore
COPY ./.formatter.exs ./.formatter.exs
COPY ./mix.exs ./mix.exs
COPY ./mix.lock ./mix.lock

RUN mix local.hex --force
RUN mix local.rebar --force

ENV MIX_ENV="prod"
ENV ICB_TOKEN=$ICB_TOKEN

RUN mix deps.get
RUN mix compile --warnings-as-errors
RUN mix release

# << app >>
FROM alpine:3.19 AS app

RUN apk add --update bash openssl libgcc libstdc++ ncurses-libs

WORKDIR /app
COPY --from=builder /app/_build/prod/ ./_build/prod/

RUN ls -la /app/_build/prod/rel/icb/erts-14.2.2/bin/

ENV ICB_TOKEN=$ICB_TOKEN

EXPOSE 8081/tcp 8082/tcp

CMD ["/app/_build/prod/rel/icb/bin/icb", "start"]
