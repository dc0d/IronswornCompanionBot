defmodule ICM.Oracles do
  @moduledoc false

  use Agent
  alias ICB.Core.Model.Dice

  def start_link(_initial_value) do
    {:ok, ironsworn_oracles_prompts_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_prompts.json") |> File.read()

    {:ok, ironsworn_oracles_prompts} = ironsworn_oracles_prompts_json_string |> Jaxon.decode()

    {:ok, ironsworn_oracles_character_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_character.json") |> File.read()

    {:ok, ironsworn_oracles_character} = ironsworn_oracles_character_json_string |> Jaxon.decode()

    Agent.start_link(
      fn ->
        %{
          ironsworn_oracles_prompts: ironsworn_oracles_prompts,
          ironsworn_oracles_character: ironsworn_oracles_character
        }
      end,
      name: __MODULE__
    )
  end

  def action_theme() do
    {:ok, [action_r, theme_r]} = Dice.roll("2d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_prompts: ironsworn_oracles_prompts} = _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_prompts

      #

      %{"Oracle Table" => action_table} =
        oracles
        |> Enum.find(fn oracle -> oracle["Name"] == "Action" end)

      %{"Oracle Table" => theme_table} =
        oracles
        |> Enum.find(fn oracle -> oracle["Name"] == "Theme" end)

      #

      %{"Description" => action} =
        action_table
        |> Enum.find(fn record -> record["Chance"] == action_r end)

      %{"Description" => theme} =
        theme_table
        |> Enum.find(fn record -> record["Chance"] == theme_r end)

      #

      {action, theme}
    end)
  end

  def make_npc() do
    {:ok, [role_r, goal_r, descriptor_r, disposition_r]} = Dice.roll("4d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_character: ironsworn_oracles_character} = _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_character

      #

      %{"Oracle Table" => role_table} =
        oracles
        |> Enum.find(fn oracle -> oracle["Name"] == "Role" end)

      %{"Oracle Table" => goal_table} =
        oracles
        |> Enum.find(fn oracle -> oracle["Name"] == "Goal" end)

      %{"Oracle Table" => descriptor_table} =
        oracles
        |> Enum.find(fn oracle -> oracle["Name"] == "Descriptor" end)

      %{"Oracle Table" => disposition_table} =
        oracles
        |> Enum.find(fn oracle -> oracle["Name"] == "Disposition" end)

      #

      %{"Description" => role} =
        role_table
        |> Enum.find(fn record -> record["Chance"] >= role_r end)

      %{"Description" => goal} =
        goal_table
        |> Enum.find(fn record -> record["Chance"] >= goal_r end)

      %{"Description" => descriptor} =
        descriptor_table
        |> Enum.find(fn record -> record["Chance"] >= descriptor_r end)

      %{"Description" => disposition} =
        disposition_table
        |> Enum.find(fn record -> record["Chance"] >= disposition_r end)

      %{role: role, goal: goal, descriptor: descriptor, disposition: disposition}
    end)
  end
end
