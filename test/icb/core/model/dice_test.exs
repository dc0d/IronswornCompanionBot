defmodule ICB.Core.Model.DiceTest do
  use ExUnit.Case
  alias ICB.Core.Model.Dice
  doctest Dice

  test "3d6" do
    {:ok, result} = Dice.roll("3d6")
    assert 3 == Enum.count(result)
    assert Enum.all?(result, fn n -> n >= 1 and n <= 6 end)
  end

  test "1d6,2d10,1d12,6d20" do
    {:ok, result} = Dice.roll("1d6,2d10,1d12,6d20")
    assert 10 == Enum.count(result)
    assert Enum.all?(result, fn n -> n >= 1 and n <= 20 end)
  end
end
