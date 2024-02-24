defmodule ICB.Oracles do
  @moduledoc false

  use Agent
  require Logger
  alias ICB.Core.Model.Dice

  def start_link(_initial_value) do
    Agent.start_link(
      fn ->
        %{
          ironsworn_oracles_prompts: read_ironsworn_oracles_prompts(),
          ironsworn_oracles_character: read_ironsworn_oracles_character(),
          ironsworn_oracles_names: read_ironsworn_oracles_names(),
          ironsworn_oracles_settlement: read_ironsworn_oracles_settlement(),
          ironsworn_oracles_turning_point: read_ironsworn_oracles_turning_point()
        }
      end,
      name: __MODULE__
    )
  end

  def action_theme do
    {:ok, [action_r, theme_r]} = Dice.roll("2d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_prompts: ironsworn_oracles_prompts} = _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_prompts

      %{"Oracle Table" => action_table} = find_oracle(oracles, "Action")
      %{"Oracle Table" => theme_table} = find_oracle(oracles, "Theme")

      %{"Description" => action} = find_choice(action_table, action_r)
      %{"Description" => theme} = find_choice(theme_table, theme_r)

      {action, theme}
    end)
  end

  def make_npc do
    {:ok, [role_r, goal_r, descriptor_r, disposition_r]} = Dice.roll("4d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_character: ironsworn_oracles_character} = _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_character

      %{"Oracle Table" => role_table} = find_oracle(oracles, "Role")
      %{"Oracle Table" => goal_table} = find_oracle(oracles, "Goal")
      %{"Oracle Table" => descriptor_table} = find_oracle(oracles, "Descriptor")
      %{"Oracle Table" => disposition_table} = find_oracle(oracles, "Disposition")

      %{"Description" => role} = find_choice(role_table, role_r)
      %{"Description" => goal} = find_choice(goal_table, goal_r)
      %{"Description" => descriptor} = find_choice(descriptor_table, descriptor_r)
      %{"Description" => disposition} = find_choice(disposition_table, disposition_r)

      %{role: role, goal: goal, descriptor: descriptor, disposition: disposition}
    end)
  end

  # turing point

  def reveal_challenge_rank do
    table_name = "Challenge Rank"
    {:ok, [rolled]} = Dice.roll("1d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_turning_point: ironsworn_oracles_turning_point} =
                               _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_turning_point
      %{"Oracle Table" => oracle_table} = find_oracle(oracles, table_name)
      %{"Description" => answer} = find_choice(oracle_table, rolled)

      answer
    end)
  end

  def reveal_combat_action do
    table_name = "Combat Action"
    {:ok, [rolled]} = Dice.roll("1d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_turning_point: ironsworn_oracles_turning_point} =
                               _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_turning_point
      %{"Oracle Table" => oracle_table} = find_oracle(oracles, table_name)
      %{"Description" => answer} = find_choice(oracle_table, rolled)

      answer
    end)
  end

  def reveal_major_plot_twist do
    table_name = "Major Plot Twist"
    {:ok, [rolled]} = Dice.roll("1d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_turning_point: ironsworn_oracles_turning_point} =
                               _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_turning_point
      %{"Oracle Table" => oracle_table} = find_oracle(oracles, table_name)
      %{"Description" => answer} = find_choice(oracle_table, rolled)

      answer
    end)
  end

  def reveal_mystic_backlash do
    table_name = "Mystic Backlash"
    {:ok, [rolled]} = Dice.roll("1d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_turning_point: ironsworn_oracles_turning_point} =
                               _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_turning_point
      %{"Oracle Table" => oracle_table} = find_oracle(oracles, table_name)
      %{"Description" => answer} = find_choice(oracle_table, rolled)

      answer
    end)
  end

  # settlement

  def settlement_quick do
    {:ok, [settlement_trouble_r, prefix_r, suffix_r]} = Dice.roll("3d100")

    Agent.get(__MODULE__, fn %{ironsworn_oracles_settlement: ironsworn_oracles_settlement} =
                               _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_settlement

      %{"Oracle Table" => settlement_trouble} = find_oracle(oracles, "Settlement Trouble")
      quick_settlement_name = find_oracle(oracles, "Quick Settlement Name")

      %{"Oracles" => quick_settlement_name_oracles} = quick_settlement_name
      %{"Oracle Table" => prefix_table} = find_oracle(quick_settlement_name_oracles, "Prefix")
      %{"Oracle Table" => suffix_table} = find_oracle(quick_settlement_name_oracles, "Suffix")

      %{"Description" => trouble} = find_choice(settlement_trouble, settlement_trouble_r)
      %{"Description" => prefix} = find_choice(prefix_table, prefix_r)
      %{"Description" => suffix} = find_choice(suffix_table, suffix_r)

      %{trouble: trouble, prefix: prefix, suffix: suffix}
    end)
  end

  # make names

  def name_ironlander do
    {:ok, [rolled]} = Dice.roll("1d200")
    table_name = "Ironlander Names"

    give_name(rolled, table_name)
  end

  def name_elf do
    {:ok, [rolled]} = Dice.roll("1d100")
    table_name = "Elf Names"

    give_name(rolled, table_name)
  end

  def name_giant do
    {:ok, [rolled]} = Dice.roll("1d100")
    table_name = "Giant Names"

    give_name(rolled, table_name)
  end

  def name_varou do
    {:ok, [rolled]} = Dice.roll("1d100")
    table_name = "Varou Names"

    give_name(rolled, table_name)
  end

  def name_troll do
    {:ok, [rolled]} = Dice.roll("1d100")
    table_name = "Troll Names"

    give_name(rolled, table_name)
  end

  defp give_name(rolled, table_name) do
    Agent.get(__MODULE__, fn %{ironsworn_oracles_names: ironsworn_oracles_names} = _state ->
      %{"Oracles" => oracles} = ironsworn_oracles_names
      %{"Oracle Table" => table_of_names} = find_oracle(oracles, table_name)
      %{"Description" => name} = find_choice(table_of_names, rolled)

      name
    end)
  end

  # loader functions

  defp read_ironsworn_oracles_prompts do
    {:ok, ironsworn_oracles_prompts_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_prompts.json") |> File.read()

    {:ok, ironsworn_oracles_prompts} = ironsworn_oracles_prompts_json_string |> Jaxon.decode()

    ironsworn_oracles_prompts
  end

  defp read_ironsworn_oracles_character do
    {:ok, ironsworn_oracles_character_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_character.json") |> File.read()

    {:ok, ironsworn_oracles_character} = ironsworn_oracles_character_json_string |> Jaxon.decode()

    ironsworn_oracles_character
  end

  defp read_ironsworn_oracles_names do
    {:ok, ironsworn_oracles_names_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_names.json") |> File.read()

    {:ok, ironsworn_oracles_names} = ironsworn_oracles_names_json_string |> Jaxon.decode()

    ironsworn_oracles_names
  end

  defp read_ironsworn_oracles_settlement do
    {:ok, ironsworn_oracles_settlement_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_settlement.json") |> File.read()

    {:ok, ironsworn_oracles_settlement} =
      ironsworn_oracles_settlement_json_string |> Jaxon.decode()

    ironsworn_oracles_settlement
  end

  defp read_ironsworn_oracles_turning_point do
    {:ok, ironsworn_oracles_turning_point_json_string} =
      Application.app_dir(:icb, "/priv/ironsworn_oracles_turning_point.json") |> File.read()

    {:ok, ironsworn_oracles_turning_point} =
      ironsworn_oracles_turning_point_json_string |> Jaxon.decode()

    ironsworn_oracles_turning_point
  end

  # support

  defp find_oracle(oracles, name) do
    oracles
    |> Enum.find(fn oracle -> oracle["Name"] == name end)
  end

  defp find_choice(table, rolled) do
    table
    |> Enum.find(fn record -> record["Chance"] >= rolled end)
  end
end
