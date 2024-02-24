defmodule ICB.Core.Model.Support do
  @moduledoc false

  def ask_the_oracle_callback_prefix, do: "ORCL::ATO::"

  def bot_username do
    {:ok,
     %Telegex.Type.User{
       username: username
     }} = Telegex.Instance.fetch_me()

    username
  end
end
