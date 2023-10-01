defmodule ICB.Chains.NameCommand do
  @moduledoc false

  @callback get_oracles() :: atom()

  defmacro __using__({:command, command_name}) do
    quote do
      @tg_command unquote(command_name)
      @tg_command_name "/#{Atom.to_string(@tg_command)}"

      use Telegex.Chain, {:command, @tg_command}
      require Logger

      @impl true
      def match?(%{text: @command <> _ = text, chat: %{type: "private"}} = _message, _context)
          when text != nil do
        true
      end

      @impl true
      def match?(_message, _context), do: false

      @impl true
      def handle(%{chat: chat, text: @tg_command_name <> _} = _update, context) do
        name = apply(get_oracles(), @tg_command, [])

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

      def get_oracles(), do: ICB.Oracles

      defoverridable get_oracles: 0
    end
  end
end
