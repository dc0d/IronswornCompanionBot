import Config

config :telegex,
  token: System.get_env("ICB_TOKEN"),
  caller_adapter: {Finch, [receive_timeout: 5 * 1000]}

config :logger_json, :backend,
  metadata: :all,
  json_encoder: Jason,
  formatter: LoggerJSON.Formatters.GoogleCloudLogger

config :logger, backends: [LoggerJSON]

import_config "#{config_env()}.exs"
