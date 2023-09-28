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
        %Telegex.Type.BotCommand{command: "ask_the_oracle", description: "ask the oracle"},
        %Telegex.Type.BotCommand{
          command: "action_theme",
          description: "roll on action/theme oracles"
        },
        %Telegex.Type.BotCommand{
          command: "make_npc",
          description: "make NPC"
        }
      ]

      Telegex.set_my_commands(commands)
      |> inspect()
      |> Logger.info()
    end)
  end
end
