defmodule ICB.Core.Model.Odds do
  @moduledoc false

  def list, do: [:small_chance, :unlikely, :fifty_fifty, :likely, :almost_certain]

  @spec resolve(atom(), keyword) :: :no | :yes
  def resolve(odds, opts \\ []) do
    opts = Keyword.merge([rand: &__MODULE__.rand/1], opts)
    rand = Keyword.get(opts, :rand)

    case {odds, rand.(100)} do
      {:small_chance, chance} when chance <= 10 -> :yes
      {:unlikely, chance} when chance <= 25 -> :yes
      {:fifty_fifty, chance} when chance <= 50 -> :yes
      {:likely, chance} when chance <= 75 -> :yes
      {:almost_certain, chance} when chance <= 90 -> :yes
      _ -> :no
    end
  end

  @spec to_string(:almost_certain | :fifty_fifty | :likely | :small_chance | :unlikely) ::
          String.t()
  def to_string(:small_chance), do: "Small Chance"
  def to_string(:unlikely), do: "Unlikely"
  def to_string(:fifty_fifty), do: "50/50"
  def to_string(:likely), do: "Likely"
  def to_string(:almost_certain), do: "Almost Certain"

  @spec parse(String.t()) ::
          :almost_certain | :fifty_fifty | :likely | :small_chance | :unlikely
  def parse("Small Chance"), do: :small_chance
  def parse("Unlikely"), do: :unlikely
  def parse("50/50"), do: :fifty_fifty
  def parse("Likely"), do: :likely
  def parse("Almost Certain"), do: :almost_certain

  @spec rand(pos_integer) :: pos_integer
  def rand(n), do: :rand.uniform(n)
end
