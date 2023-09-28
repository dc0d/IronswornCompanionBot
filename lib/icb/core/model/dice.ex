defmodule ICB.Core.Model.Dice do
  @moduledoc false

  alias ICB.Core.Model.Dice.Parser
  require Logger

  def roll(dice_expr, opts \\ []) when is_binary(dice_expr) do
    opts = Keyword.merge([rand: fn n -> :rand.uniform(n) end], opts)
    rand = Keyword.get(opts, :rand)

    with parsed <- Parser.dice(dice_expr),
         {:ok, {:ok, pairs, _, _, _, _}} <- check_expr(parsed) do
      {:ok, roll_number(pairs, rand: rand)}
    else
      error ->
        Logger.error(inspect(error))
        {:error, error}
    end
  end

  #

  defp roll_number([count, n | rest], rand: rand) do
    for(_ <- 1..count, do: rand.(n)) ++ roll_number(rest, rand: rand)
  end

  defp roll_number([], rand: _) do
    []
  end

  #

  defp check_expr({:ok, _pairs, _, _, _, _} = expr) do
    {:ok, expr}
  end

  defp check_expr(dice_expr) do
    Logger.error(inspect({:unknown_dice_expr, dice_expr}))
    {:error, :unknown_dice_expr}
  end
end
