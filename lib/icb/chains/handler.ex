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
    ICB.Chains.NameTroll,
    ICB.Chains.SettlementQuick,
    ICB.Chains.RevealChallengeRank,
    ICB.Chains.RevealCombatAction,
    ICB.Chains.RevealMajorPlotTwist,
    ICB.Chains.RevealMysticBacklash
  ])
end
