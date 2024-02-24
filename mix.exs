defmodule ICB.MixProject do
  use Mix.Project

  def project do
    [
      app: :icb,
      version: "0.3.0",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      elixirc_paths: elixirc_paths(Mix.env()),
      aliases: aliases(),
      test_coverage: [
        summary: [threshold: 1],
        ignore_modules: [
          ICB.Application
        ]
      ]
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {ICB.Application, []}
    ]
  end

  defp deps do
    [
      {:credo, "~> 1.7.4", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.4.3", only: [:dev], runtime: false},
      {:telegex, "~> 1.4.2"},
      {:finch, "~> 0.18.0"},
      {:multipart, "~> 0.4.0"},
      {:nimble_parsec, "~> 1.4.0"},
      {:jaxon, "~> 2.0.8"}
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp aliases do
    [
      compile: ["compile --warning-as-errors"],
      test: ["test --no-start"]
    ]
  end
end
