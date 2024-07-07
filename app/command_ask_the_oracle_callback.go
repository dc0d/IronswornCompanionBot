package app

import (
	"fmt"
	"strings"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"
	callbackquery "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers/filters/callbackquery"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type AskTheOracleCommandCallback struct {
	handlers.CallbackQuery
	logger *prelude.Logger
	dice   model.DiceService
}

func NewAskTheOracleCommandCallback(logger *prelude.Logger, dice model.DiceService) ext.Handler {
	result := AskTheOracleCommandCallback{
		logger: logger,
		dice:   dice,
	}
	result.CallbackQuery = handlers.NewCallback(callbackquery.Prefix(askTheOracleCallbackPrefix), result.Execute)

	return result
}

func (cmd AskTheOracleCommandCallback) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("ask_the_oracle command received", "RCVD", ctx.CallbackQuery)

	oddsText := strings.TrimPrefix(ctx.CallbackQuery.Data, askTheOracleCallbackPrefix)
	actualOdds := cmd.dice.RollD100()
	answer := model.ResolveOdds(actualOdds, oddsText)
	resolutionSign := "✅"
	if !answer {
		resolutionSign = "❌"
	}

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, fmt.Sprintf("%v %v %v", answer, resolutionSign, oddsText), nil)
	if err != nil {
		return fmt.Errorf("ask_the_oracle callback failed: %w", err)
	}

	_, err = bot.DeleteMessage(ctx.EffectiveChat.Id, ctx.CallbackQuery.Message.GetMessageId(), nil)
	if err != nil {
		return fmt.Errorf("ask_the_oracle delete keyboard failed: %w", err)
	}

	return nil
}
