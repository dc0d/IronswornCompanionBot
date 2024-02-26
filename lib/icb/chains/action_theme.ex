defmodule ICB.Chains.ActionTheme do
  @moduledoc false

  use Telegex.Chain, {:command, :action_theme}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/action_theme" <> _} = _update, context) do
    {action, theme} = ICB.Oracles.action_theme()

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{action} #{theme} 📦"
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
