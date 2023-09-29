defmodule ICB.Chains.NameIronlander do
  @moduledoc false

  use Telegex.Chain, {:command, :name_ironlander}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{type: "private"}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/name_ironlander" <> _} = _update, context) do
    name =
      ICB.Oracles.name_ironlander()

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
