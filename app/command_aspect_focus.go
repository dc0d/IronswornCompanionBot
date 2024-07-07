package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type AspectFocusCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewAspectFocusCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := AspectFocusCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("aspect_focus", result.Execute)

	return result
}

func (cmd AspectFocusCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("aspect_focus command received", "RCVD", ctx.Message)

	aspectFocus := cmd.orcSvc.AspectFocus()

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, fmt.Sprintf("%v %v 🔎", aspectFocus.Aspect, aspectFocus.Focus), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
