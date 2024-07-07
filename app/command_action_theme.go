package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type ActionThemeCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewActionThemeCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := ActionThemeCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("action_theme", result.Execute)

	return result
}

func (cmd ActionThemeCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("action_theme command received", "RCVD", ctx.Message)

	actionTheme := cmd.orcSvc.ActionTheme()

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, fmt.Sprintf("%v %v 📦", actionTheme.Action, actionTheme.Theme), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
