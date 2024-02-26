defmodule ICB.Chains.AskTheOracle do
  @moduledoc false

  use Telegex.Chain, {:command, :ask_the_oracle}
  require Logger

  alias ICB.Core.Model.Odds
  alias ICB.Core.Model.Support
  alias Telegex.Type.InlineKeyboardButton
  alias Telegex.Type.InlineKeyboardMarkup

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/ask_the_oracle" <> _} = _update, context) do
    markup = %InlineKeyboardMarkup{
      inline_keyboard:
        Odds.list()
        |> Enum.map(fn odds ->
          %InlineKeyboardButton{
            text: Odds.to_string(odds),
            callback_data: Support.ask_the_oracle_callback_prefix() <> Odds.to_string(odds)
          }
        end)
        |> Enum.chunk_every(2)
        |> Enum.map(fn row -> Enum.to_list(row) end)
        |> Enum.to_list()
    }

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "Choose the odds: 🔮",
          reply_markup: markup
        }
    }

    {:done, context}
  end

  @impl true
  def handle(update, context) do
    Logger.warning("unhandled update", %{update: update, context: context})
    {:done, context}
  end
end
