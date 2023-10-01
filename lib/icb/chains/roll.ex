defmodule ICB.Chains.Roll do
  @moduledoc false

  use Telegex.Chain, {:command, :roll}
  require Logger

  @ironsworn_dice_expr "1d6,2d10"

  @impl true
  def match?(%{text: @command <> _ = text, chat: %{type: "private"}} = _message, _context)
      when text != nil do
    true
  end

  @impl true
  def match?(_message, _context), do: false

  @impl true
  def handle(%{chat: _chat, text: "/roll"} = update, context) do
    ironsworn_handler(update, context)
  end

  @impl true
  def handle(%{chat: chat, text: "/roll " <> n_text} = update, context) do
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

  @impl true
  def handle(update, context) do
    Logger.warning(inspect(%{signal: :unandled_update, update: update, context: context}))
    {:done, context}
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

  defp get_dice(), do: ICB.Core.Model.Dice
end
