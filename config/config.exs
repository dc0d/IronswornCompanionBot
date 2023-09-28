import Config

config :telegex,
  token: System.get_env("ICB_TOKEN"),
  caller_adapter: {Finch, [receive_timeout: 5 * 1000]}

config :logger, :console, metadata: [:error_code, :file, :line]
