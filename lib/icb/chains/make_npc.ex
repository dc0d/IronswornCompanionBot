defmodule ICB.Chains.MakeCharacter do
  @moduledoc false

  use Telegex.Chain, {:command, :make_npc}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/make_npc" <> _} = _update, context) do
    %{role: role, goal: goal, descriptor: descriptor, disposition: disposition} =
      ICB.Oracles.make_npc()

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{disposition} #{descriptor} #{role} 🎯 #{goal}"
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
