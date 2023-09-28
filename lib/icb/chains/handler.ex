defmodule ICB.Chains.Handler do
  @moduledoc false

  use Telegex.Chain.Handler

  pipeline([
    ICB.Chains.Start,
    ICB.Chains.Roll,
    ICB.Chains.AskTheOracle,
    ICB.Chains.AskTheOracleCallback,
    ICB.Chains.ActionTheme,
    ICB.Chains.MakeCharacter
  ])
end
