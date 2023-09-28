defmodule ICB.Core.Model.Dice.ParserTest do
  use ExUnit.Case
  alias ICB.Core.Model.Dice.Parser
  doctest Parser

  test "3d6" do
    assert {:ok, [3, 6], "", %{}, {1, 0}, 3} == Parser.dice("3d6")
  end

  test "1d6,2d10,1d12,6d20" do
    assert {:ok, [1, 6, 2, 10, 1, 12, 6, 20], "", %{}, {1, 0}, 18} ==
             Parser.dice("1d6,2d10,1d12,6d20")
  end
end
