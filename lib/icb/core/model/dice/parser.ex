defmodule ICB.Core.Model.Dice.Parser do
  @moduledoc false
  import NimbleParsec

  single_dice =
    integer(min: 1)
    |> ignore(string("d"))
    |> integer(min: 1)

  next_dice =
    ignore(string(","))
    |> concat(single_dice)

  expr =
    single_dice
    |> times(next_dice, min: 0)

  defparsec(:dice, expr, export_metadata: true)
end
