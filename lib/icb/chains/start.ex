defmodule ICB.Chains.Start do
  @moduledoc false

  use Telegex.Chain, {:command, :start}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{type: "private"}} = _message, _context)
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

          You can see a list of command by:

          -   Typing '/' to bring up the command menu or
          -   Pressing the commands menu button.
          """
        }
    }

    {:done, context}
  end

  @impl true
  def handle(update, context) do
    Logger.warning(%{signal: :unandled_update, update: update, context: context})
    {:done, %{}}
  end
end
