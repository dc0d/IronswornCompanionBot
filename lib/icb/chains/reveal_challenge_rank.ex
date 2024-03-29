defmodule ICB.Chains.RevealChallengeRank do
  @moduledoc false

  use Telegex.Chain, {:command, :reveal_challenge_rank}
  require Logger

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: chat, text: "/reveal_challenge_rank" <> _} = _update, context) do
    answer = ICB.Oracles.reveal_challenge_rank()

    answer_sign =
      case answer do
        "Troublesome" -> "🟢🟢🟢🟢🔴"
        "Dangerous" -> "🟢🟢🟢🔴🔴"
        "Formidable" -> "🟢🟢🔴🔴🔴"
        "Extreme" -> "🟢🔴🔴🔴🔴"
        "Epic" -> "🔴🔴🔴🔴🔴"
        _ -> "👽"
      end

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{answer} #{answer_sign}"
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
