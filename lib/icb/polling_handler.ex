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
          description: "roll on action/theme oracles"
        },
        %Telegex.Type.BotCommand{
          command: "name_ironlander",
          description: "gives ironlander name"
        },
        %Telegex.Type.BotCommand{command: "name_elf", description: "gives elf name"},
        %Telegex.Type.BotCommand{command: "name_giant", description: "gives giant name"},
        %Telegex.Type.BotCommand{command: "name_varou", description: "gives varou name"},
        %Telegex.Type.BotCommand{command: "name_troll", description: "gives troll name"}
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
