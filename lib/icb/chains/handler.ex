defmodule ICB.Chains.Handler do
  @moduledoc false

  use Telegex.Chain.Handler

  pipeline([
    ICB.Chains.Start,
    ICB.Chains.Roll,
    ICB.Chains.AskTheOracle,
    ICB.Chains.AskTheOracleCallback,
    ICB.Chains.ActionTheme,
    ICB.Chains.MakeCharacter,
    ICB.Chains.NameIronlander,
    ICB.Chains.NameElf,
    ICB.Chains.NameGiant,
    ICB.Chains.NameVarou,
    ICB.Chains.NameTroll
  ])
end
