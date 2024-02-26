defmodule ICB.PollingHandler do
  @moduledoc false

  use Telegex.Polling.GenHandler
  require Logger

  alias ICB.Chains.Handler

  @impl true
  def on_boot do
    fetch_me_result = Telegex.Instance.fetch_me()
    delete_webhook_result = Telegex.delete_webhook()

    is_token_provided =
      case System.get_env("ICB_TOKEN") do
        nil -> false
        "" -> false
        _ -> true
      end

    Logger.info("on boot setup", %{
      is_token_provided: is_token_provided,
      delete_webhook_result: delete_webhook_result,
      fetch_me_result: fetch_me_result
    })

    {:ok, _} = delete_webhook_result

    {:ok, user} = fetch_me_result
    Logger.info("Bot (@#{user.username}) is working (polling)")

    set_commands()

    %Telegex.Polling.Config{
      timeout: 3,
      interval: 513,
      limit: 10
    }
  end

  @impl true
  def on_update(update) do
    log_update(update)

    Handler.call(update, %ICB.Chains.Context{bot: Telegex.Instance.bot()})
  end

  @impl true
  def on_failure(update, err) do
    Logger.error("uncaught polling update", %{update: update, error: err})
  end

  defp set_commands do
    Task.start(fn ->
      Process.sleep(5000)

      commands = [
        %Telegex.Type.BotCommand{command: "start", description: "starts the bot"},
        %Telegex.Type.BotCommand{
          command: "roll",
          description: "rolls the dice - defaults to ironsworn dice, also accepts 1d20,3d6 format"
        },
        %Telegex.Type.BotCommand{
          command: "make_npc",
          description: "make NPC"
        },
        %Telegex.Type.BotCommand{command: "ask_the_oracle", description: "ask the oracle"},
        %Telegex.Type.BotCommand{
          command: "action_theme",
          description: "set action & theme"
        },
        %Telegex.Type.BotCommand{
          command: "name_ironlander",
          description: "give ironlander name"
        },
        %Telegex.Type.BotCommand{command: "name_elf", description: "give elf name"},
        %Telegex.Type.BotCommand{command: "name_giant", description: "give giant name"},
        %Telegex.Type.BotCommand{command: "name_varou", description: "give varou name"},
        %Telegex.Type.BotCommand{command: "name_troll", description: "give troll name"},
        %Telegex.Type.BotCommand{
          command: "settlement_quick",
          description: "make quick settlement"
        },
        %Telegex.Type.BotCommand{
          command: "reveal_challenge_rank",
          description: "reveal challenge rank"
        },
        %Telegex.Type.BotCommand{
          command: "reveal_combat_action",
          description: "reveal combat action"
        },
        %Telegex.Type.BotCommand{
          command: "reveal_major_plot_twist",
          description: "reveal major plot twist"
        },
        %Telegex.Type.BotCommand{
          command: "reveal_mystic_backlash",
          description: "reveal mystic backlash"
        }
      ]

      case Telegex.set_my_commands(commands) do
        {:error, error} -> Logger.error("set_my_commands errored", %{error: error})
        _ -> :ok
      end
    end)
  end

  defp log_update(update) do
    Logger.info("incoming update", %{update: update})

    :ok
  end
end
