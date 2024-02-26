defmodule ICB.Chains.Start do
  @moduledoc false

  use Telegex.Chain, {:command, :start}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: _text} = _update, context) do
    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: """
          Let's get on with our journey! 🎲

          This bot is designed to help you play Ironsworn, a free tabletop role-playing game by Shawn Tomkin. It can help you with dice rolls, generate NPCs, and answer questions using the oracles.

          You can choose a command from the menu or type a command directly. Also, typing '/' brings up the command menu.
          """
        }
    }

    {:done, context}
  end

  @impl true
  def handle(update, context) do
    Logger.warning("unhandled update", %{update: update, context: context})
    {:done, %{}}
  end
end
