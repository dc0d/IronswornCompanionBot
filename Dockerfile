FROM elixir:1.15 AS builder

ARG ICB_TOKEN

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

RUN mix deps.get
RUN mix compile --warnings-as-errors
RUN mix release

ENV ICB_TOKEN=$ICB_TOKEN

EXPOSE 8081/tcp 8082/tcp

CMD ["./_build/prod/rel/icb/bin/icb", "start"]
