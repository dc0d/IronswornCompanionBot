defmodule ICB.Chains.SettlementQuick do
  @moduledoc false

  use Telegex.Chain, {:command, :settlement_quick}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/settlement_quick" <> _} = _update, context) do
    %{trouble: trouble, prefix: prefix, suffix: suffix} = ICB.Oracles.settlement_quick()

    prefix = String.trim(prefix, "-")

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{prefix}#{suffix} 🏘️ #{trouble}"
        }
    }

    {:done, context}
  end

  @impl true
  def handle(update, context) do
    Logger.warning(inspect(%{signal: :unandled_update, update: update, context: context}))
    {:done, context}
  end
end
