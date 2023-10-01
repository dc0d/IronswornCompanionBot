defmodule ICB.Chains.RevealChallengeRank do
  @moduledoc false

  use Telegex.Chain, {:command, :reveal_challenge_rank}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{type: "private"}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/reveal_challenge_rank" <> _} = _update, context) do
    answer = ICB.Oracles.reveal_challenge_rank()

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{answer}"
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
