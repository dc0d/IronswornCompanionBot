defmodule ICB.Chains.NameVarou do
  @moduledoc false

  use Telegex.Chain, {:command, :name_varou}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{type: "private"}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/name_varou" <> _} = _update, context) do
    name =
      ICB.Oracles.name_varou()

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "❝ #{name} ❞"
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
