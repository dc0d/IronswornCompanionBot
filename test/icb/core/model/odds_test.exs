defmodule ICB.Core.Model.OddsTest do
  use ExUnit.Case

  alias ICB.Core.Model.Odds
  doctest Odds

  ExUnit.Case.register_describe_attribute(__MODULE__, :test_fixtures, accumulate: true)

  describe "odds for ask the oracle action" do
    @test_fixtures %{
      input: %{
        odds: :small_chance,
        rand: &__MODULE__.rand_10/1
      },
      expected_output: :yes
    }

    @test_fixtures %{
      input: %{
        odds: :small_chance,
        rand: &__MODULE__.rand_11/1
      },
      expected_output: :no
    }

    @test_fixtures %{
      input: %{
        odds: :unlikely,
        rand: &__MODULE__.rand_25/1
      },
      expected_output: :yes
    }

    @test_fixtures %{
      input: %{
        odds: :unlikely,
        rand: &__MODULE__.rand_26/1
      },
      expected_output: :no
    }

    @test_fixtures %{
      input: %{
        odds: :fifty_fifty,
        rand: &__MODULE__.rand_50/1
      },
      expected_output: :yes
    }

    @test_fixtures %{
      input: %{
        odds: :fifty_fifty,
        rand: &__MODULE__.rand_51/1
      },
      expected_output: :no
    }

    @test_fixtures %{
      input: %{
        odds: :likely,
        rand: &__MODULE__.rand_75/1
      },
      expected_output: :yes
    }

    @test_fixtures %{
      input: %{
        odds: :likely,
        rand: &__MODULE__.rand_76/1
      },
      expected_output: :no
    }

    @test_fixtures %{
      input: %{
        odds: :almost_certain,
        rand: &__MODULE__.rand_90/1
      },
      expected_output: :yes
    }

    @test_fixtures %{
      input: %{
        odds: :almost_certain,
        rand: &__MODULE__.rand_91/1
      },
      expected_output: :no
    }

    test "resolve should give the expected yes/no response around the boundaries", context do
      for %{input: %{odds: odds, rand: rand}, expected_output: yes_no} <-
            context.registered.test_fixtures do
        assert yes_no == Odds.resolve(odds, rand: rand)
      end
    end
  end

  describe "odds to text" do
    @test_fixtures %{input: :small_chance, expected_output: "Small Chance"}
    @test_fixtures %{input: :unlikely, expected_output: "Unlikely"}
    @test_fixtures %{input: :fifty_fifty, expected_output: "50/50"}
    @test_fixtures %{input: :likely, expected_output: "Likely"}
    @test_fixtures %{input: :almost_certain, expected_output: "Almost Certain"}

    test "to_string should generate the expected text", context do
      for %{input: odds, expected_output: text} <-
            context.registered.test_fixtures do
        assert text == Odds.to_string(odds)
      end
    end
  end

  describe "parse text to odds" do
    @test_fixtures %{input: "Small Chance", expected_output: :small_chance}
    @test_fixtures %{input: "Unlikely", expected_output: :unlikely}
    @test_fixtures %{input: "50/50", expected_output: :fifty_fifty}
    @test_fixtures %{input: "Likely", expected_output: :likely}
    @test_fixtures %{input: "Almost Certain", expected_output: :almost_certain}

    test "parse should return the expected odds", context do
      for %{input: text, expected_output: odds} <-
            context.registered.test_fixtures do
        assert odds == Odds.parse(text)
      end
    end
  end

  #

  def rand_10(_n), do: 10
  def rand_11(_n), do: 11
  def rand_25(_n), do: 25
  def rand_26(_n), do: 26
  def rand_50(_n), do: 50
  def rand_51(_n), do: 51
  def rand_75(_n), do: 75
  def rand_76(_n), do: 76
  def rand_90(_n), do: 90
  def rand_91(_n), do: 91
end
