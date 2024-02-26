defmodule ICB.Chains.RevealMysticBacklash do
  @moduledoc false

  use Telegex.Chain, {:command, :reveal_mystic_backlash}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/reveal_mystic_backlash" <> _} = _update, context) do
    answer = ICB.Oracles.reveal_mystic_backlash()

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{answer} 🧿"
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
