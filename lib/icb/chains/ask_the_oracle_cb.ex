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
          message: %{
            message_id: message_id,
            chat: %{id: chat_id}
          }
        } =
          _callback_query,
        context
      ) do
    case Telegex.delete_message(chat_id, message_id) do
      {:error, error} -> Logger.error("delete_message", %{error: error})
      _ -> :ok
    end

    odds_text =
      data
      |> String.replace_prefix(Support.ask_the_oracle_callback_prefix(), "")

    resolution =
      odds_text
      |> Odds.parse()
      |> Odds.resolve()

    resolution_sign =
      case resolution do
        :yes -> "✅"
        :no -> "❌"
      end

    resolution_text =
      resolution
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

      case Telegex.send_message(chat_id, "#{resolution_text} #{resolution_sign} #{odds_text}") do
        {:error, error} -> Logger.error("send_message", %{error: error})
        _ -> :ok
      end
    end)

    {:done, context}
  end

  @impl true
  def handle(callback_query, context) do
    Logger.warning(
      "unhandled callback_query",
      %{callback_query: callback_query, context: context}
    )

    {:done, context}
  end
end
