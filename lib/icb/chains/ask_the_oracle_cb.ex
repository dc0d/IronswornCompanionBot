defmodule ICB.Chains.AskTheOracleCallback do
  @moduledoc false

  alias ICB.Core.Model.Support
  use Telegex.Chain, {:callback_query, prefix: Support.ask_the_oracle_callback_prefix()}
  require Logger
  alias ICB.Core.Model.Odds
  alias ICB.Core.Model.Resolution

  @impl true
  def handle(
        %Telegex.Type.CallbackQuery{
          data: data,
          id: id,
          message: %Telegex.Type.Message{
            message_id: message_id,
            chat: %Telegex.Type.Chat{id: chat_id}
          }
        } =
          _callback_query,
        context
      ) do
    Telegex.delete_message(chat_id, message_id)
    |> inspect()
    |> Logger.info()

    odds_text =
      data
      |> String.replace_prefix(Support.ask_the_oracle_callback_prefix(), "")

    resolution =
      odds_text
      |> Odds.parse()
      |> Odds.resolve()
      |> Resolution.to_string()

    context = %{
      context
      | payload: %{
          method: "answerCallbackQuery",
          callback_query_id: id
        }
    }

    Task.start(fn ->
      Process.sleep(361)

      Telegex.send_message(chat_id, "#{resolution} 🎲 #{odds_text}")
      |> inspect()
      |> Logger.info()
    end)

    {:done, context}
  end

  @impl true
  def handle(callback_query, context) do
    Logger.warning(%{
      signal: :unandled_callback_query,
      callback_query: callback_query,
      context: context
    })

    {:done, context}
  end
end
