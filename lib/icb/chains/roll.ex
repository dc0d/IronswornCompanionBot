defmodule ICB.Chains.Roll do
  @moduledoc false

  use Telegex.Chain, {:command, :roll}
  require Logger

  alias ICB.Core.Model.Support

  @ironsworn_dice_expr "1d6,2d10"

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: _chat, text: "/roll" <> _ = text} = update, context) do
    bot_username = Support.bot_username()

    case text do
      "/roll" ->
        ironsworn_handler(update, context)

      "/roll@" <> ^bot_username ->
        ironsworn_handler(update, context)

      "/roll " <> n_text ->
        other_dice_handler(update, context, n_text)

      "/roll@" <> ^bot_username <> " " <> n_text ->
        other_dice_handler(update, context, n_text)

      _ ->
        Logger.warning("unhandled update", %{update: update, context: context})
        {:done, context}
    end
  end

  @impl true
  def handle(update, context) do
    Logger.warning("unhandled update", %{update: update, context: context})
    {:done, context}
  end

  defp other_dice_handler(%{chat: chat} = update, context, n_text) do
    case get_dice().roll(n_text) do
      {:ok, result} ->
        context = %{
          context
          | payload: %{
              method: "sendMessage",
              chat_id: chat.id,
              text: "#{inspect(result, charlists: :as_lists)}"
            }
        }

        {:done, context}

      {:error, _} ->
        ironsworn_handler(update, context)
    end
  end

  defp ironsworn_handler(%{chat: chat} = _update, context) do
    {:ok, [d6, d10_1, d10_2]} = get_dice().roll(@ironsworn_dice_expr)
    rolled_challenge = "#{d10_1} - #{d10_2}"
    rolled_challenge = rolled_challenge <> if d6 > d10_1 && d6 > d10_2, do: " 💪", else: ""
    rolled_challenge = rolled_challenge <> if d10_1 == d10_2, do: " 🎁", else: ""

    context = %{
      context
      | payload: %{
          method: "sendMessage",
          chat_id: chat.id,
          text: "#{d6} 🎲 #{rolled_challenge}"
        }
    }

    {:done, context}
  end

  defp get_dice, do: ICB.Core.Model.Dice
end
