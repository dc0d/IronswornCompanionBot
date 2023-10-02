defmodule ICB.PollingHandler do
  @moduledoc false

  use Telegex.Polling.Handler
  require Logger

  @impl true
  def on_boot do
    {:ok, user} = Telegex.Instance.get_me()
    {:ok, _} = Telegex.delete_webhook()

    Logger.info("Bot (@#{user.username}) is working (polling)")

    set_commands()

    %Telegex.Polling.Config{
      timeout: 3,
      interval: 500
    }
  end

  @impl true
  def on_update(update) do
    log_update(update)

    ICB.Chains.Handler.call(update, %ICB.Chains.Context{bot: Telegex.Instance.me()})
  end

  defp set_commands do
    Task.start(fn ->
      Process.sleep(3000)

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
        {:error, error} -> error |> inspect() |> Logger.error()
        _ -> :ok
      end
    end)
  end

  defp log_update(update) do
    case update do
      %{
        callback_query: %{
          data: data,
          message: %{
            from: %{
              first_name: first_name,
              last_name: last_name,
              username: username,
              id: id
            },
            text: text
          }
        }
      } ->
        Logger.info("from [#{first_name}] [#{last_name}] (@#{username}) (#{id}): #{text} #{data}")

      %{
        message: %{
          from: %{
            first_name: first_name,
            last_name: last_name,
            username: username,
            id: id
          },
          text: text
        }
      } ->
        Logger.info("from [#{first_name}] [#{last_name}] (@#{username}) (#{id}): #{text}")

      _ ->
        :ok
    end
  end
end
