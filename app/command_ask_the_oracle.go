package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type AskTheOracleCommand struct {
	handlers.Command
	logger *prelude.Logger
}

func NewAskTheOracleCommand(logger *prelude.Logger) ext.Handler {
	result := AskTheOracleCommand{
		logger: logger,
	}
	result.Command = handlers.NewCommand("ask_the_oracle", result.Execute)

	return result
}

func (cmd AskTheOracleCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("ask_the_oracle command received", "RCVD", ctx.Message)

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, "Choose the odds: 🔮", &gotgbot.SendMessageOpts{
		ParseMode: "Markdown",
		ReplyMarkup: gotgbot.InlineKeyboardMarkup{
			InlineKeyboard: [][]gotgbot.InlineKeyboardButton{
				{
					{
						Text:         model.SmallChance.String(),
						CallbackData: askTheOracleCallbackPrefix + model.SmallChance.String(),
					},
					{
						Text:         model.Unlikely.String(),
						CallbackData: askTheOracleCallbackPrefix + model.Unlikely.String(),
					},
				},
				{
					{
						Text:         model.FiftyFifty.String(),
						CallbackData: askTheOracleCallbackPrefix + model.FiftyFifty.String(),
					},
					{
						Text:         model.Likely.String(),
						CallbackData: askTheOracleCallbackPrefix + model.Likely.String(),
					},
				},
				{
					{
						Text:         model.AlmostCertain.String(),
						CallbackData: askTheOracleCallbackPrefix + model.AlmostCertain.String(),
					},
				},
			},
		},
	})
	if err != nil {
		return fmt.Errorf("failed to reply to ask_the_oracle command: %w", err)
	}

	return nil
}

const askTheOracleCallbackPrefix = "ORCL::ATO::"
